'use client';

import { useState, useEffect } from 'react';
import { Sidebar } from '@/components/Sidebar';
import { EmailList } from '@/components/EmailList';
import { EmailPreview } from '@/components/EmailPreview';
import { Button } from '@/components/ui/button';
import { api, Email, EmailDetail } from '@/lib/api';
import { RefreshCw } from 'lucide-react';

interface GenerateDraftResponse {
  draft_id: number;
  content: string;
}

interface FetchUnreadResponse {
  fetched: boolean;
}

export default function InboxPage() {
  const [emails, setEmails] = useState<Email[]>([]);
  const [selectedEmail, setSelectedEmail] = useState<EmailDetail | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isFetching, setIsFetching] = useState(false);
  const [isGenerating, setIsGenerating] = useState(false);

  useEffect(() => {
    loadEmails();
  }, []);

  const loadEmails = async () => {
    try {
      setIsLoading(true);
      const data: Email[] = await api.listEmails();
      setEmails(data);
    } catch (error) {
      console.error('Failed to load emails:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleSelectEmail = async (email: Email) => {
    try {
      const detail: EmailDetail = await api.getEmail(email.id);
      setSelectedEmail(detail);
    } catch (error) {
      console.error('Failed to load email detail:', error);
    }
  };

  const handleFetchUnread = async () => {
    try {
      setIsFetching(true);

      const result: FetchUnreadResponse = await api.fetchUnread();

      if (!result.fetched) {
        alert("No unread emails found");
      }

      await loadEmails();
    } catch (error) {
      console.error('Failed to fetch unread emails:', error);
      alert('Failed to fetch unread emails');
    } finally {
      setIsFetching(false);
    }
  };

  const handleGenerateDraft = async (emailId: number) => {
    try {
      setIsGenerating(true);

      const result: GenerateDraftResponse = await api.generateDraft(
        emailId,
        'friendly'
      );

      alert(`Draft generated! Draft ID: ${result.draft_id}`);

      window.location.href = `/drafts?draftId=${result.draft_id}`;
    } catch (error) {
      console.error('Failed to generate draft:', error);
      alert('Failed to generate draft');
    } finally {
      setIsGenerating(false);
    }
  };

  return (
    <div className="flex h-screen">
      <Sidebar />
      <div className="flex flex-1 flex-col">
        <div className="flex h-16 items-center justify-between border-b px-6">
          <h2 className="text-2xl font-semibold">Inbox</h2>
          <Button
            onClick={handleFetchUnread}
            disabled={isFetching}
            variant="outline"
          >
            <RefreshCw className={`mr-2 h-4 w-4 ${isFetching ? 'animate-spin' : ''}`} />
            Fetch Unread
          </Button>
        </div>

        <div className="flex flex-1 overflow-hidden">
          <div className="w-80 border-r">
            <EmailList
              emails={emails}
              selectedEmailId={selectedEmail?.id ?? null}
              onSelectEmail={handleSelectEmail}
              isLoading={isLoading}
            />
          </div>

          <div className="flex-1 overflow-y-auto p-6">
            <EmailPreview
              email={selectedEmail}
              onGenerateDraft={handleGenerateDraft}
              isLoading={isGenerating}
            />
          </div>
        </div>

      </div>
    </div>
  );
}
