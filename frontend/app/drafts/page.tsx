'use client';

import { useState, useEffect, Suspense } from 'react';
import { useSearchParams } from 'next/navigation';
import { Sidebar } from '@/components/Sidebar';
import { DraftEditor } from '@/components/DraftEditor';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { api, Draft } from '@/lib/api';
import { FileText, Sparkles } from 'lucide-react';
import { format } from 'date-fns';

function DraftsContent() {
  const searchParams = useSearchParams();
  const [drafts, setDrafts] = useState<Draft[]>([]);
  const [selectedDraft, setSelectedDraft] = useState<Draft | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);

  useEffect(() => {
    loadDrafts();
  }, []);

  useEffect(() => {
    const draftId = searchParams.get('draftId');
    if (draftId && drafts.length > 0) {
      const draft = drafts.find((d) => d.id === parseInt(draftId));
      if (draft) {
        setSelectedDraft(draft);
      }
    }
  }, [searchParams, drafts]);

  const loadDrafts = async () => {
    try {
      setIsLoading(true);
      const data = await api.listDrafts();
      setDrafts(data);
      
      // If we have a draftId in URL but no selected draft, try to load it
      const draftId = searchParams.get('draftId');
      if (draftId && !selectedDraft) {
        const draft = data.find((d) => d.id === parseInt(draftId));
        if (draft) {
          setSelectedDraft(draft);
        } else {
          // Try to fetch it directly if not in list
          try {
            const draft = await api.getDraft(parseInt(draftId));
            setSelectedDraft(draft);
          } catch (e) {
            console.error('Failed to load draft:', e);
          }
        }
      }
    } catch (error) {
      console.error('Failed to load drafts:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleSave = async (id: number, content: string) => {
    try {
      setIsSaving(true);
      await api.updateDraft(id, content);
      await loadDrafts();
      if (selectedDraft?.id === id) {
        const updated = await api.getDraft(id);
        setSelectedDraft(updated);
      }
    } catch (error) {
      console.error('Failed to save draft:', error);
      alert('Failed to save draft');
    } finally {
      setIsSaving(false);
    }
  };

  const handleApprove = async (id: number) => {
    try {
      await api.approveDraft(id);
      await loadDrafts();
      if (selectedDraft?.id === id) {
        const updated = await api.getDraft(id);
        setSelectedDraft(updated);
      }
    } catch (error) {
      console.error('Failed to approve draft:', error);
      alert('Failed to approve draft');
    }
  };

  const handleSend = async (id: number) => {
    try {
      const result = await api.sendDraft(id);
      alert(`Email sent successfully! Gmail ID: ${result.sent_gmail_id}`);
      // Reload drafts to show updated status
      await loadDrafts();
      // Update selected draft if it's the one we just sent
      if (selectedDraft?.id === id) {
        const updated = await api.getDraft(id);
        setSelectedDraft(updated);
      }
    } catch (error) {
      console.error('Failed to send draft:', error);
      alert('Failed to send draft');
    }
  };

  return (
    <div className="flex h-screen">
      <Sidebar />
      <div className="flex flex-1 flex-col">
        <div className="flex h-16 items-center border-b px-6">
          <h2 className="text-2xl font-semibold">Drafts</h2>
        </div>
        <div className="flex flex-1 overflow-hidden">
          <div className="w-80 border-r p-4">
            {isLoading ? (
              <div className="text-center text-muted-foreground">Loading drafts...</div>
            ) : drafts.length === 0 ? (
              <div className="text-center text-muted-foreground">
                <FileText className="mx-auto mb-4 h-12 w-12 opacity-50" />
                <p>No drafts yet</p>
                <p className="mt-2 text-sm">
                  Generate a draft from an email in your inbox
                </p>
              </div>
            ) : (
              <div className="space-y-2">
                {drafts.map((draft) => (
                  <Card
                    key={draft.id}
                    className={`cursor-pointer transition-colors hover:bg-accent ${
                      selectedDraft?.id === draft.id ? 'bg-accent' : ''
                    }`}
                    onClick={() => setSelectedDraft(draft)}
                  >
                    <CardHeader className="pb-2">
                      <CardTitle className="text-sm">Draft #{draft.id}</CardTitle>
                    </CardHeader>
                    <CardContent>
                      <p className="text-xs text-muted-foreground">
                        {format(new Date(draft.created_at), 'PPp')}
                      </p>
                      <p className="mt-1 text-xs text-muted-foreground">
                        Status: {draft.status || 'draft'}
                      </p>
                    </CardContent>
                  </Card>
                ))}
              </div>
            )}
          </div>
          <div className="flex-1 overflow-y-auto p-6">
            <DraftEditor
              draft={selectedDraft}
              onSave={handleSave}
              onApprove={handleApprove}
              onSend={handleSend}
              isLoading={isSaving}
            />
          </div>
        </div>
      </div>
    </div>
  );
}

export default function DraftsPage() {
  return (
    <Suspense
      fallback={
        <div className="flex h-screen items-center justify-center">
          Loading drafts...
        </div>
      }
    >
      <DraftsContent />
    </Suspense>
  );
}

