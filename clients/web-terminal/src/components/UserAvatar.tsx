import React, { useState, useRef, useEffect } from 'react';
import { List, Settings, LogOut } from 'lucide-react';
import { Button } from './ui/button';

// 用户信息类型
export interface UserInfo {
  id: string;
  name: string;
  avatar: string;
  sessionLimit: number;
  activeSessions: number;
}

interface UserAvatarProps {
  userInfo: UserInfo;
  onListSessions: () => void;
  onSettings?: () => void;
  onLogout?: () => void;
}

export const UserAvatar: React.FC<UserAvatarProps> = ({ 
  userInfo, 
  onListSessions, 
  onSettings,
  onLogout 
}) => {
  const [showUserMenu, setShowUserMenu] = useState(false);
  const userMenuRef = useRef<HTMLDivElement>(null);
  
  // 点击外部区域关闭下拉菜单
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (userMenuRef.current && !userMenuRef.current.contains(event.target as Node)) {
        setShowUserMenu(false);
      }
    };
    
    // 添加事件监听器
    document.addEventListener('mousedown', handleClickOutside);
    
    // 清理函数
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, []);

  return (
    <div className="relative" ref={userMenuRef}>
      <Button
        onClick={() => setShowUserMenu(!showUserMenu)}
        variant="ghost"
        size="sm"
        className="h-9 w-9 p-0 hover:scale-105 transition-all duration-200 shadow-md"
        title="User profile"
      >
        <img 
          src={userInfo.avatar} 
          alt={userInfo.name} 
          className="w-full h-full object-cover rounded-full border-2 border-primary/30 hover:border-primary/50 transition-all duration-200 shadow-lg"
        />
      </Button>
      
      {/* User Menu Dropdown */}
      {showUserMenu && (
        <div className="absolute right-0 top-full mt-1 bg-card border border-border rounded-lg shadow-lg z-500 py-2 w-56">
          {/* User Info */}
          <div className="px-4 py-3 border-b border-border">
            <div className="flex items-center space-x-3">
              <img 
                src={userInfo.avatar} 
                alt={userInfo.name} 
                className="w-10 h-10 object-cover rounded-full border border-primary/30"
              />
              <div>
                <div className="font-medium text-sm">{userInfo.name}</div>
                <div className="text-xs text-muted-foreground">{userInfo.id}</div>
              </div>
            </div>
            {/* Session Usage */}
            <div className="mt-3">
              <div className="flex justify-between text-xs mb-1">
                <span className="text-muted-foreground">Session Usage</span>
                <span className="font-medium">{userInfo.activeSessions}/{userInfo.sessionLimit}</span>
              </div>
              <div className="w-full bg-muted rounded-full h-2">
                <div 
                  className="bg-gradient-to-r from-primary to-accent h-2 rounded-full transition-all duration-300 ease-in-out"
                  style={{ width: `${(userInfo.activeSessions / userInfo.sessionLimit) * 100}%` }}
                ></div>
              </div>
            </div>
          </div>
          
          {/* Menu Items */}
          <div className="py-1">
            <button
              onClick={() => {
                onListSessions();
                setShowUserMenu(false);
              }}
              className="w-full flex items-center space-x-3 px-4 py-2 text-sm hover:bg-primary/10 transition-colors"
            >
              <List size={14} className="text-purple-500" />
              <span>My Sessions</span>
            </button>
            <button
              onClick={() => {
                if (onSettings) {
                  onSettings();
                }
                setShowUserMenu(false);
              }}
              className="w-full flex items-center space-x-3 px-4 py-2 text-sm hover:bg-primary/10 transition-colors"
            >
              <Settings size={14} className="text-blue-500" />
              <span>Settings</span>
            </button>
          </div>
          
          {/* Logout */}
          <div className="border-t border-border py-1">
            <button
              onClick={() => {
                if (onLogout) {
                  onLogout();
                }
                setShowUserMenu(false);
              }}
              className="w-full flex items-center space-x-3 px-4 py-2 text-sm text-red-500 hover:bg-red-500/10 transition-colors"
            >
              <LogOut size={14} className="text-red-500" />
              <span>Logout</span>
            </button>
          </div>
        </div>
      )}
    </div>
  );
};