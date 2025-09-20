export interface Snippet {
  id: string
  title: string
  language: string
  content: string
  description: string
  passwordProtected: boolean
  password?: string | null
  createdAt: string // ISO datetime
  updatedAt: string // ISO datetime
  expiresAt?: string | null // ISO datetime or null
  author?: string | null
  viewCount: number
  tags?: string | null
  visibility: number
}

export interface CreateSnippetRequest {
  title: string
  language: string
  content: string
  description?: string | null
  password?: string | null
  expiresInSeconds?: number | null
  author?: string | null
  tags?: string | null
  visibility?: number | null
}

export interface CreateSnippetReply {
  success: boolean
  message: string
  snippet?: Snippet | null
}

export interface ErrorResponse {
  error: string
  message: string
}

export interface GetSnippetReply {
  snippet: Snippet
  passwordRequired: boolean
}
