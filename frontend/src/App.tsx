import React, { useState, useRef } from 'react'
import { SearchBar } from './components/SearchBar'
import { FileDataTable } from './components/FileDataTable'
import { UploadModal } from './components/UploadModal'
import { Title } from './components/Title'
import { useFiles } from './hooks/useFiles'
import { useUpload } from './hooks/useUpload'
import { fileService } from './services/api'
import { errorService } from './services/errorService'
import { Card, CardContent } from './components/ui/card'

function App() {
  const { files, searchTerm, setSearchTerm, loadFiles } = useFiles()
  const { uploadProgress, uploadFile, cancelUpload } = useUpload(() => loadFiles(searchTerm))
  const [dragState, setDragState] = useState(false)
  const dragCount = useRef(0)

  const grabFile = async (fileId: string, fileName: string) => {
    try {
      const link = document.createElement('a')
      link.href = fileService.getDownloadUrl(fileId)
      link.download = fileName
      document.body.appendChild(link)
      link.click()
      document.body.removeChild(link)
    } catch (error) {
      errorService.handleDownloadError(error)
    }
  }

  const stopEvent = (e: React.DragEvent) => {
    e.preventDefault()
    e.stopPropagation()
  }

  const onDragEnter = (e: React.DragEvent) => {
    stopEvent(e)
    dragCount.current++
    if (e.dataTransfer.items && e.dataTransfer.items.length > 0) {
      setDragState(true)
    }
  }

  const onDragLeave = (e: React.DragEvent) => {
    stopEvent(e)
    dragCount.current--
    if (dragCount.current === 0) {
      setDragState(false)
    }
  }

  const onDragOver = stopEvent

  const onDrop = (e: React.DragEvent) => {
    stopEvent(e)
    setDragState(false)
    dragCount.current = 0

    if (e.dataTransfer.files && e.dataTransfer.files.length > 0) {
      const file = e.dataTransfer.files[0]
      uploadFile(file)
    }
  }
  
  return (
    <div 
      className="min-h-screen flex items-center justify-center p-4"
      onDragEnter={onDragEnter}
      onDragLeave={onDragLeave}
      onDragOver={onDragOver}
      onDrop={onDrop}
    >
      <div className="w-full max-w-4xl">
        <Title />
        <SearchBar 
          value={searchTerm}
          onChange={setSearchTerm}
        />
        
        <Card className={`transition-all ${dragState ? 'ring-2 ring-primary bg-primary/5' : ''}`}>
          <CardContent className="p-0">
            {files.length > 0 ? (
              <FileDataTable 
                files={files}
                onDownload={grabFile}
              />
            ) : (
              <div className="text-center py-12 text-muted-foreground">
                <p className="text-lg font-medium">Drag and Drop here</p>
              </div>
            )}
          </CardContent>
        </Card>

        <UploadModal
          isOpen={uploadProgress?.isUploading || false}
          onClose={cancelUpload}
          progress={uploadProgress?.progress || 0}
          fileName={uploadProgress?.file_name || ''}
          onCancel={cancelUpload}
        />
      </div>
    </div>
  )
}

export default App 