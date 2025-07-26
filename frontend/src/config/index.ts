export const config = {
  api: {
    baseUrl: '/api', // its just for dev env, its a proxy for vite
    endpoints: {
      files: '/files',
      upload: '/upload',
      notifyUpload: '/notify-upload',
      download: '/download'
    }
  },
  locale: {
    dateFormat: 'en-US' as const,
  }
} as const
