import React, { useEffect } from 'react';
import { List, User, Clock, Folder, Terminal, RefreshCw, X, Check } from 'lucide-react';
import { Button } from './ui/button';

interface Session {
  id: string;
  userId: string;
  shellType: string;
  status: string;
  terminalSize: { columns: number; rows: number };
  workingDirectory: string;
  createdAt: number;
  updatedAt: number;
}

// 用户信息接口，用于生成用户头像
interface UserInfo {
  id: string;
  name: string;
  avatar: string;
}

interface SessionListModalProps {
  isOpen: boolean;
  sessions: Session[];
  currentSessionId: string;
  title?: string;
  onClose: () => void;
}

export const SessionListModal: React.FC<SessionListModalProps> = ({ isOpen, sessions, currentSessionId, title = 'Terminal Sessions', onClose }) => {
  const [filteredSessions, setFilteredSessions] = React.useState<Session[]>([]);
  const [filterStatus, setFilterStatus] = React.useState('all');
  const [sortBy, setSortBy] = React.useState('createdAt-desc');
  const [sessionStats, setSessionStats] = React.useState({ active: 0, terminated: 0 });
  
  // 生成用户头像的辅助函数
  const generateUserAvatar = (userId: string): string => {
    // 从userId中提取前两个字符作为头像文字
    const initials = userId.substring(0, 2).toUpperCase();
    // 使用ui-avatars API生成头像
    return `https://ui-avatars.com/api/?name=${encodeURIComponent(initials)}&background=random&color=fff&size=40`;
  };
  
  // 生成用户信息的辅助函数
  const getUserInfo = (userId: string): UserInfo => {
    return {
      id: userId,
      name: `User ${userId.substring(0, 8)}`,
      avatar: generateUserAvatar(userId)
    };
  };
  
  // 当sessions、filterStatus或sortBy变化时，更新过滤和排序后的session列表
  useEffect(() => {
    let result = [...sessions];
    
    // 应用状态过滤，忽略大小写
    if (filterStatus !== 'all') {
      result = result.filter(session => session.status?.toUpperCase() === filterStatus);
    }
    
    // 应用排序
    const [field, direction] = sortBy.split('-');
    result.sort((a, b) => {
      const aValue = a[field as keyof Session];
      const bValue = b[field as keyof Session];
      
      if (direction === 'asc') {
        return aValue > bValue ? 1 : -1;
      } else {
        return aValue < bValue ? 1 : -1;
      }
    });
    
    setFilteredSessions(result);
  }, [sessions, filterStatus, sortBy]);
  
  // 当sessions变化时，更新统计信息
  useEffect(() => {
    const activeCount = sessions.filter(session => session.status === 'ACTIVE').length;
    const terminatedCount = sessions.filter(session => session.status === 'TERMINATED').length;
    setSessionStats({ active: activeCount, terminated: terminatedCount });
  }, [sessions]);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50">
      <div className="bg-card border border-border rounded-xl shadow-2xl w-full max-w-3xl max-h-[90vh] overflow-hidden">
        <div className="flex flex-col md:flex-row md:items-center justify-between p-6 border-b border-border gap-3">
          <h3 className="text-lg font-semibold text-foreground flex items-center gap-2">
            <List size={18} className="text-primary" />
            {title}
          </h3>
          
          {/* Session Stats */}
          <div className="flex items-center gap-3">
            <div className="flex items-center gap-1">
              <span className="px-2 py-0.5 rounded-full text-xs font-medium bg-green-500/20 text-green-500">
                Active
              </span>
              <span className="text-sm font-medium">{sessionStats.active}</span>
            </div>
            <div className="flex items-center gap-1">
              <span className="px-2 py-0.5 rounded-full text-xs font-medium bg-red-500/20 text-red-500">
                Terminated
              </span>
              <span className="text-sm font-medium">{sessionStats.terminated}</span>
            </div>
          </div>
          
          <Button 
            onClick={onClose}
            variant="ghost" 
            size="sm" 
            className="h-8 w-8 p-0 hover:bg-muted"
          >
            ×
          </Button>
        </div>
        
        {/* Filter and Sort Controls */}
        <div className="p-4 border-b border-border bg-muted/20">
          <div className="flex flex-wrap gap-3 items-center">
            <div className="flex items-center gap-2">
              <label className="text-sm font-medium text-foreground">Status:</label>
              <select 
                className="px-3 py-1.5 bg-background border border-input rounded-md text-sm text-foreground focus:border-primary focus:outline-none"
                value={filterStatus}
                onChange={(e) => setFilterStatus(e.target.value)}
              >
                <option value="all">All</option>
                <option value="ACTIVE">Active</option>
                <option value="TERMINATED">Terminated</option>
              </select>
            </div>
            <div className="flex items-center gap-2">
              <label className="text-sm font-medium text-foreground">Sort by:</label>
              <select 
                className="px-3 py-1.5 bg-background border border-input rounded-md text-sm text-foreground focus:border-primary focus:outline-none"
                value={sortBy}
                onChange={(e) => setSortBy(e.target.value)}
              >
                <option value="createdAt-desc">Created (Newest First)</option>
                <option value="createdAt-asc">Created (Oldest First)</option>
                <option value="updatedAt-desc">Updated (Newest First)</option>
                <option value="updatedAt-asc">Updated (Oldest First)</option>
              </select>
            </div>
          </div>
        </div>
        
        <div className="overflow-y-auto max-h-[calc(90vh-160px)] p-6">
          {filteredSessions.length === 0 ? (
            <div className="text-center py-12 text-muted-foreground">
              <div className="flex justify-center mb-4">
                <List size={48} className="text-muted-foreground/50" />
              </div>
              <p className="text-lg font-medium mb-2">No sessions found</p>
              <p className="text-sm">Try adjusting your filter criteria</p>
            </div>
          ) : (
            <div className="space-y-4">
              {filteredSessions.map((session) => {
                const userInfo = getUserInfo(session.userId);
                return (
                  <div key={session.id} className="bg-muted/50 rounded-lg p-5 border border-border hover:border-primary/30 hover:shadow-lg transition-all duration-200">
                    <div className="flex flex-col md:flex-row md:items-start justify-between gap-4">
                      {/* Session Header with User Info */}
                      <div className="flex-1">
                        <div className="flex items-center gap-3 flex-wrap mb-3">
                          {/* User Avatar */}
                          <img 
                            src={userInfo.avatar} 
                            alt={userInfo.name} 
                            className="w-8 h-8 object-cover rounded-full border border-primary/30 shadow-sm"
                          />
                          
                          {/* Session ID and Status */}
                          <div className="flex items-center gap-2 flex-wrap">
                            <span className="font-mono text-primary text-sm">{session.id}</span>
                            <span className={`px-2 py-0.5 rounded-full text-xs font-medium flex items-center gap-1 ${session.status === 'ACTIVE' ? 'bg-green-500/20 text-green-500' : 'bg-red-500/20 text-red-500'}`}>
                              {session.status === 'ACTIVE' ? <Check size={10} /> : <X size={10} />}
                              {session.status}
                            </span>
                            {session.id === currentSessionId && (
                              <span className="px-2 py-0.5 rounded-full text-xs font-medium bg-primary/20 text-primary flex items-center gap-1">
                                <RefreshCw size={10} className="animate-spin" />
                                Current
                              </span>
                            )}
                          </div>
                        </div>
                        
                        {/* Session Details Grid */}
                        <div className="grid grid-cols-1 md:grid-cols-3 gap-3 mt-3 text-sm">
                          <div className="flex items-center gap-2">
                            <Terminal size={14} className="text-primary/70" />
                            <span className="text-muted-foreground">Shell:</span>
                            <span className="font-medium">{session.shellType}</span>
                          </div>
                          <div className="flex items-center gap-2">
                            <svg className="w-4 h-4 text-primary/70" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4" />
                            </svg>
                            <span className="text-muted-foreground">Size:</span>
                            <span className="font-mono">{session.terminalSize?.columns}×{session.terminalSize?.rows}</span>
                          </div>
                          <div className="flex items-center gap-2">
                            <Folder size={14} className="text-primary/70" />
                            <span className="text-muted-foreground">Dir:</span>
                            <span className="font-mono truncate max-w-[200px]">{session.workingDirectory}</span>
                          </div>
                          <div className="flex items-center gap-2">
                            <User size={14} className="text-primary/70" />
                            <span className="text-muted-foreground">User:</span>
                            <span className="font-medium">{userInfo.name}</span>
                          </div>
                          <div className="flex items-center gap-2">
                            <Clock size={14} className="text-primary/70" />
                            <span className="text-muted-foreground">Created:</span>
                            <span>{new Date(session.createdAt).toLocaleString()}</span>
                          </div>
                          <div className="flex items-center gap-2">
                            <Clock size={14} className="text-primary/70" />
                            <span className="text-muted-foreground">Updated:</span>
                            <span>{new Date(session.updatedAt).toLocaleString()}</span>
                          </div>
                        </div>
                      </div>
                      
                      {/* Session Actions */}
                      <div className="flex flex-col gap-2 text-xs text-muted-foreground min-w-[120px]">
                        {/* Session Duration */}
                        <div className="flex flex-col gap-1">
                          <div className="font-medium text-foreground flex items-center gap-1">
                            <Clock size={12} />
                            Duration
                          </div>
                          <div>{Math.floor((Date.now() - session.createdAt) / 1000 / 60)}m</div>
                        </div>
                      </div>
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </div>
        
        <div className="p-6 border-t border-border flex justify-between items-center bg-muted/20">
          <div className="text-sm text-muted-foreground">
            Showing <span className="font-medium text-foreground">{filteredSessions.length}</span> of <span className="font-medium text-foreground">{sessions.length}</span> sessions
          </div>
          <Button 
            onClick={onClose}
            variant="outline" 
            className="flex-1 max-w-[150px]"
          >
            Close
          </Button>
        </div>
      </div>
    </div>
  );
};