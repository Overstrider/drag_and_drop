import React from 'react'
import {
  ColumnDef,
  flexRender,
  getCoreRowModel,
  useReactTable,
} from "@tanstack/react-table"
import { Download } from 'lucide-react'

import { Button } from "./ui/button"
import {
  Table,
  TableBody,
  TableCell,
  TableRow,
} from "./ui/table"
import { FileInfo } from '../types'
import { formatFileSize } from '../utils/formatFileSize'
import { formatDate } from '../utils/formatDate'

interface FileDataTableProps {
  files: FileInfo[]
  onDownload: (fileId: string, fileName: string) => void
}

export function FileDataTable({ files, onDownload }: FileDataTableProps) {
  const cols: ColumnDef<FileInfo>[] = [
    {
      accessorKey: "original_name",
      header: "File Name",
      cell: ({ row }) => {
        const file = row.original
        return (
          <div className="font-medium group relative flex items-center gap-2">
            <span>{row.getValue("original_name") as string}</span>
            <Button
              variant="noShadow"
              size="sm"
              onClick={() => onDownload(file.file_id, file.original_name)}
              className="opacity-0 group-hover:opacity-100 transition-opacity duration-200 p-1 h-8 w-8"
            >
              <Download className="h-4 w-4" />
            </Button>
          </div>
        )
      },
    },
    {
      accessorKey: "size",
      header: "Size",
      cell: ({ row }) => {
        const size = row.getValue("size") as number
        return <div>{formatFileSize(size)}</div>
      },
    },
    {
      accessorKey: "upload_date",
      header: "Upload Date",
      cell: ({ row }) => {
        const date = row.getValue("upload_date") as string
        return <div>{formatDate(date)}</div>
      },
    },

  ]

  const table = useReactTable({
    data: files,
    columns: cols,
    getCoreRowModel: getCoreRowModel(),
  })

  return (
    <div className="w-full">
      <Table>
        <TableBody>
          {table.getRowModel().rows?.length ? (
            table.getRowModel().rows.map((row) => (
              <TableRow
                key={row.id}
                data-state={row.getIsSelected() && "selected"}
              >
                {row.getVisibleCells().map((cell) => (
                  <TableCell key={cell.id}>
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </TableCell>
                ))}
              </TableRow>
            ))
          ) : (
            <TableRow>
              <TableCell colSpan={cols.length} className="h-24 text-center">
                No files found.
              </TableCell>
            </TableRow>
          )}
        </TableBody>
      </Table>
    </div>
  )
} 