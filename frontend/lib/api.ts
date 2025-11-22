const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8000';

export interface Email {
  id: number;
  gmail_id: string;
  thread_id: string | null;
  user_email: string | null;
  sender: string | null;
  to_recipients: string | null;
  subject: string | null;
  snippet: string | null;
  has_body: boolean;
  fetched_at: string;
}

export interface EmailDetail extends Email {
  body_text: string | null;
  body_html: string | null;
  labels: string[] | null;
}

export interface Draft {
  id: number;
  email_id: number;
  content: string | null;
  tone: string | null;
  status: string | null;
  created_at: string;
}

class ApiClient {
  private getToken(): string | null {
    if (typeof window === 'undefined') return null;
    return localStorage.getItem('jwt_token');
  }

  private async request<T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<T> {
    const token = this.getToken();
    const headers: HeadersInit = {
      'Content-Type': 'application/json',
      ...options.headers,
    };

    if (token) {
      headers['Authorization'] = `Bearer ${token}`;
    }

    const url = `${API_BASE_URL}${endpoint}`;
    
    try {
      const response = await fetch(url, {
        ...options,
        headers,
      });

      if (!response.ok) {
        const errorText = await response.text().catch(() => response.statusText);
        console.error(`API Error [${response.status}]:`, errorText);
        
        if (response.status === 401) {
          // Token expired or invalid
          localStorage.removeItem('jwt_token');
          if (typeof window !== 'undefined') {
            window.location.href = '/login';
          }
          throw new Error('Unauthorized');
        }
        throw new Error(`API error: ${response.status} ${errorText}`);
      }

      return response.json();
    } catch (error) {
      if (error instanceof TypeError && error.message.includes('fetch')) {
        console.error('Network error - is the backend running?', error);
        throw new Error(`Cannot connect to API at ${API_BASE_URL}. Make sure the backend is running on port 8000.`);
      }
      throw error;
    }
  }

  // Auth endpoints
  async startGoogleAuth() {
    return this.request<{ auth_url: string; state: string }>('/auth/google/start');
  }

  async googleCallback(code: string, state: string) {
    return this.request<{ jwt: string; email: string }>(
      `/auth/google/callback?code=${code}&state=${state}`
    );
  }

  // Email endpoints
  async listEmails(): Promise<Email[]> {
    return this.request<Email[]>('/emails');
  }

  async getEmail(id: number): Promise<EmailDetail> {
    return this.request<EmailDetail>(`/emails/${id}`);
  }

  async fetchUnread() {
    return this.request<{ fetched: boolean }>('/internal/fetch-unread', {
      method: 'POST',
    });
  }

  async fetchEmail(gmailId: string) {
    return this.request<{ ok: boolean }>(`/internal/fetch/${gmailId}`, {
      method: 'POST',
    });
  }

  // Draft endpoints
  async listDrafts(): Promise<Draft[]> {
    return this.request<Draft[]>('/drafts');
  }

  async generateDraft(emailId: number, tone?: string) {
    return this.request<{ draft_id: number; content: string }>('/drafts/generate', {
      method: 'POST',
      body: JSON.stringify({ email_id: emailId, tone }),
    });
  }

  async getDraft(id: number): Promise<Draft> {
    return this.request<Draft>(`/drafts/${id}`);
  }

  async updateDraft(id: number, content: string) {
    return this.request<{ updated: boolean }>(`/drafts/${id}`, {
      method: 'PATCH',
      body: JSON.stringify({ content }),
    });
  }

  async approveDraft(id: number) {
    return this.request<{ approved: boolean }>(`/drafts/${id}/approve`, {
      method: 'POST',
    });
  }

  async sendDraft(id: number) {
    return this.request<{ sent: boolean; sent_gmail_id: string }>(`/drafts/${id}/send`, {
      method: 'POST',
    });
  }
}

export const api = new ApiClient();

