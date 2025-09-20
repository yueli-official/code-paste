// api.ts
import axios from 'axios'
import type {
  CreateSnippetRequest,
  CreateSnippetReply,
  GetSnippetReply,
  ErrorResponse,
} from '../models/snippet'

const BASE_URL = '/api'

// Axios 实例
const api = axios.create({
  baseURL: BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
})

// ---- API 函数 ----

/**
 * 创建新的 Snippet
 */
export async function createSnippet(payload: CreateSnippetRequest): Promise<CreateSnippetReply> {
  try {
    const response = await api.post<CreateSnippetReply>('/snippets', payload)
    return response.data
  } catch (err: unknown) {
    throw err
  }
}

/**
 * 获取 Snippet
 */
export async function getSnippet(id: string, password?: string): Promise<GetSnippetReply> {
  try {
    const response = await api.get<GetSnippetReply>(`/snippets/${id}`, {
      params: { password },
    })
    return response.data
  } catch (err: unknown) {
    throw err
  }
}
