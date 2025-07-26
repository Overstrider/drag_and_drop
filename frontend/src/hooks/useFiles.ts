import { useState, useEffect, useCallback, useRef } from 'react'
import { fileService } from '../services/api'
import { errorService } from '../services/errorService'
import { FileInfo } from '../types'


export function useFiles() {
  const [fileList, setFileList] = useState<FileInfo[]>([])
  const [query, setQuery] = useState('')
  const debounceTimer = useRef<NodeJS.Timeout>()

  const loadFiles = useCallback(async (context?: string) => {
    try {
      const files = await fileService.getFiles(context)
      setFileList(files)
    } catch (error) {
      errorService.handleFilesError(error)
    }
  }, [])

  useEffect(() => {
    loadFiles()
  }, [loadFiles])

  // debounce search
  useEffect(() => {
    if (debounceTimer.current) {
      clearTimeout(debounceTimer.current)
    }

    debounceTimer.current = setTimeout(() => {
      loadFiles(query)
    }, 500)

    return () => {
      if (debounceTimer.current) {
        clearTimeout(debounceTimer.current)
      }
    }
  }, [query, loadFiles])

  return {
    files: fileList,
    searchTerm: query,
    setSearchTerm: setQuery,
    loadFiles
  }
} 