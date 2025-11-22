'use client';

import { useState, useEffect } from 'react';
import { Draft } from '@/lib/api';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Check, Send, Save } from 'lucide-react';

interface DraftEditorProps {
  draft: Draft | null;
  onSave: (id: number, content: string) => Promise<void>;
  onApprove: (id: number) => Promise<void>;
  onSend: (id: number) => Promise<void>;
  isLoading?: boolean;
}

export function DraftEditor({
  draft,
  onSave,
  onApprove,
  onSend,
  isLoading,
}: DraftEditorProps) {
  const [content, setContent] = useState(draft?.content || '');
  const [isSaving, setIsSaving] = useState(false);

  // Update content when draft changes
  useEffect(() => {
    if (draft) {
      setContent(draft.content || '');
    }
  }, [draft]);

  const handleSave = async () => {
    if (!draft) return;
    setIsSaving(true);
    try {
      await onSave(draft.id, content);
    } finally {
      setIsSaving(false);
    }
  };

  const handleApprove = async () => {
    if (!draft) return;
    await onApprove(draft.id);
  };

  const handleSend = async () => {
    if (!draft) return;
    await onSend(draft.id);
  };

  if (!draft) {
    return (
      <div className="flex h-full items-center justify-center">
        <div className="text-center text-muted-foreground">
          <p>No draft selected</p>
        </div>
      </div>
    );
  }

  const isApproved = draft.status === 'approved';

  return (
    <div className="flex h-full flex-col">
      <Card className="flex-1">
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle>Draft #{draft.id}</CardTitle>
            <div className="flex gap-2">
              <Button
                variant="outline"
                onClick={handleSave}
                disabled={isSaving || isLoading}
              >
                <Save className="mr-2 h-4 w-4" />
                Save
              </Button>
              {!isApproved && (
                <Button
                  variant="secondary"
                  onClick={handleApprove}
                  disabled={isLoading}
                >
                  <Check className="mr-2 h-4 w-4" />
                  Approve
                </Button>
              )}
              <Button
                onClick={handleSend}
                disabled={!isApproved || isLoading}
              >
                <Send className="mr-2 h-4 w-4" />
                Send
              </Button>
            </div>
          </div>
          <div className="mt-2 text-sm text-muted-foreground">
            Status: <span className="font-medium">{draft.status || 'draft'}</span>
            {isApproved && (
              <span className="ml-2 text-green-600">✓ Ready to send</span>
            )}
          </div>
        </CardHeader>
        <CardContent>
          <textarea
            value={content}
            onChange={(e) => setContent(e.target.value)}
            className="min-h-[400px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
            placeholder="Write your draft here..."
          />
        </CardContent>
      </Card>
    </div>
  );
}

