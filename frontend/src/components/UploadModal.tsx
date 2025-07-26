import React from 'react'
import { X, Upload, AlertCircle } from 'lucide-react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './ui/card'
import { Button } from './ui/button'

interface UploadModalProps {
  isOpen: boolean
  onClose: () => void
  progress: number
  fileName: string
  onCancel: () => void
}

export function UploadModal({ isOpen, onClose, progress, fileName, onCancel }: UploadModalProps) {
  if (!isOpen) return null

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
      <Card className="w-full max-w-md">
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle className="flex items-center gap-2">
              <Upload className="h-5 w-5" />
              Upload in Progress
            </CardTitle>
            <Button
              variant="ghost"
              size="sm"
              onClick={onClose}
              className="h-8 w-8 p-0"
            >
              <X className="h-4 w-4" />
            </Button>
          </div>
          <CardDescription>
            Sending: {fileName}
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          {/* Progress Bar */}
          <div className="w-full bg-muted rounded-full h-2">
            <div
              className="bg-primary h-2 rounded-full transition-all duration-300"
              style={{ width: `${progress}%` }}
            />
          </div>
          
          <div className="text-center">
            <span className="text-sm font-medium">{progress}%</span>
          </div>

          {/* Status Messages */}
          <div className="text-sm text-muted-foreground space-y-1">
            {progress < 100 && (
              <div className="flex items-center gap-2">
                <div className="w-2 h-2 bg-primary rounded-full animate-pulse" />
                Sending File...
              </div>
            )}
            {progress === 100 && (
              <div className="flex items-center gap-2 text-green-600">
                <div className="w-2 h-2 bg-green-600 rounded-full" />
                Done! Processing file...
              </div>
            )}
          </div>

          {/* Cancel Button */}
          {progress < 100 && (
            <div className="flex justify-center">
              <Button
                variant="outline"
                onClick={onCancel}
                className="flex items-center gap-2"
              >
                <X className="h-4 w-4" />
                Cancel
              </Button>
            </div>
          )}

          {/* Info */}
          <div className="flex items-start gap-2 p-3 bg-muted/50 rounded-md">
            <AlertCircle className="h-4 w-4 text-muted-foreground mt-0.5" />
            <div className="text-xs text-muted-foreground">
              The file will be processed after upload...
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  )
} 