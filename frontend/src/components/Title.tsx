import React from 'react'

export function Title() {
  return (
    <h1 className="text-6xl font-bold mb-6 text-center text-black">
      <div className="flex flex-col items-center">
        <div className="flex items-center justify-center gap-7">
          <div className="flex flex-col items-center">
            <div>DRAG</div>
            <div>DROP</div>
          </div>
          <div className="text-9xl">&</div>
        </div>
      </div>
    </h1>
  )
} 