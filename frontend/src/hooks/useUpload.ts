import { useState } from 'react'
import axios from 'axios'
import { fileService } from '../services/api'
import { errorService } from '../services/errorService'
import { UploadProgress } from '../types'

export function useUpload(onUploadComplete: () => void) {
  const [progress, setProgress] = useState<UploadProgress | null>(null)

  const uploadFile = async (file: File) => {
    const controller = new AbortController()
    
    setProgress({
      file_name: file.name,
      progress: 0,
      isUploading: true,
      cancelToken: controller
    })

    try {
      // get url
      const { file_id, presigned_url } = await fileService.getUploadUrl()

      // upload to storage
      await axios.put(presigned_url, file, {
        headers: {
          'Content-Type': file.type || 'application/octet-stream'
        },
        signal: controller.signal,
        onUploadProgress: (progressEvent: any) => {
          if (progressEvent.total) {
            const progress = Math.round((progressEvent.loaded * 100) / progressEvent.total)
            setProgress(prev => prev ? { ...prev, progress } : null)
          }
        }
      })

      // tell backend
      await fileService.notifyUpload(file_id, file.name)

      // refresh list
      onUploadComplete()
      
      // done
      setProgress(null)
    } catch (error: any) {
      if (error.name !== 'CanceledError') {
        errorService.handleUploadError(error)
      }
      setProgress(null)
    }
  }

  // stop upload
  const cancelUpload = () => {
    if (progress?.cancelToken) {
      progress.cancelToken.abort()
      setProgress(null)
    }
  }

  return {
    uploadProgress: progress,
    uploadFile,
    cancelUpload
  }
} 