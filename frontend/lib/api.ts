const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL!;

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

export interface FetchUnreadResponse {
  fetched: boolean;
}

export interface GenerateDraftResponse {
  draft_id: number;
  content: string;
}

export interface SendDraftResponse {
  sent_gmail_id: string;
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
    if (typeof window === "undefined") return null;
    return localStorage.getItem("jwt_token");
  }

  private async request<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
    const token = this.getToken();

    // FIXED: use Record<string, string>
    const headers: Record<string, string> = {
      "Content-Type": "application/json",
      ...(options.headers as Record<string, string>),
    };

    if (token) {
      headers["Authorization"] = `Bearer ${token}`;
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
          localStorage.removeItem("jwt_token");
          if (typeof window !== "undefined") {
            window.location.href = "/login";
          }
          throw new Error("Unauthorized");
        }

        throw new Error(`API error: ${response.status} ${errorText}`);
      }

      return response.json();
    } catch (error: any) {
      if (error instanceof TypeError && error.message.includes("fetch")) {
        console.error("Network error:", error);
        throw new Error(
          `Cannot connect to API at ${API_BASE_URL}. Ensure backend is running.`
        );
      }
      throw error;
    }
  }

  // Auth
  async startGoogleAuth() {
    return this.request<{ auth_url: string; state: string }>("/auth/google/start");
  }

  async googleCallback(code: string, state: string) {
    return this.request<{ jwt: string; email: string }>(
      `/auth/google/callback?code=${code}&state=${state}`
    );
  }

  // Emails
  async listEmails(): Promise<Email[]> {
    return this.request("/emails");
  }

  async getEmail(id: number): Promise<EmailDetail> {
    return this.request(`/emails/${id}`);
  }

 async fetchUnread(): Promise<FetchUnreadResponse> {
  return this.request<FetchUnreadResponse>("/internal/fetch-unread", { method: "POST" });
}

  async fetchEmail(gmailId: string) {
    return this.request(`/internal/fetch/${gmailId}`, { method: "POST" });
  }

  // Drafts
  async listDrafts(): Promise<Draft[]> {
    return this.request("/drafts");
  }

 async generateDraft(emailId: number, tone: string): Promise<GenerateDraftResponse> {
  return this.request<GenerateDraftResponse>("/internal/generate-draft", {
    method: "POST",
    body: JSON.stringify({ email_id: emailId, tone }),
  });
}

  async getDraft(id: number): Promise<Draft> {
    return this.request(`/drafts/${id}`);
  }

  async updateDraft(id: number, content: string) {
    return this.request(`/drafts/${id}`, {
      method: "PATCH",
      body: JSON.stringify({ content }),
    });
  }

  async approveDraft(id: number) {
    return this.request(`/drafts/${id}/approve`, {
      method: "POST",
    });
  }

  async sendDraft(id: number): Promise<SendDraftResponse> {
  return this.request<SendDraftResponse>(`/internal/send-draft/${id}`, { method: "POST" });
}
}

export const api = new ApiClient();
