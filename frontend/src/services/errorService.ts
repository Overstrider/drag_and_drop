export const errorService = {
  showError: (error: any, context: string) => {
    console.error(`Error in ${context}:`, error)
    alert(`Error in ${context}`)
  },

  handleUploadError: (error: any) => {
    errorService.showError(error, 'uploading file')
  },

  handleDownloadError: (error: any) => {
    errorService.showError(error, 'downloading file')
  },

  handleFilesError: (error: any) => {
    errorService.showError(error, 'loading files')
  }
} 