import axios from 'axios'
import { config } from '../config'

const api = axios.create({
  baseURL: config.api.baseUrl
})

export const fileService = {
  // fetch files
  getFiles: async (context?: string) => {
    const params = context ? { context } : {}
    const response = await api.get(config.api.endpoints.files, { params })
    return response.data
  },

  // get upload url
  getUploadUrl: async () => {
    const response = await api.get(config.api.endpoints.upload)
    return response.data
  },

  // tell backend file is done
  notifyUpload: async (fileId: string, fileName: string) => {
    const response = await api.post(config.api.endpoints.notifyUpload, {
      file_id: fileId,
      file_name: fileName
    })
    return response.data
  },

  // build download link
  getDownloadUrl: (fileId: string) => {
    return `${config.api.baseUrl}${config.api.endpoints.download}/${fileId}`
  }
}

export default api 