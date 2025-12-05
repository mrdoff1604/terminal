import React from 'react';
import { Monitor } from 'lucide-react';
import { Button } from './ui/button';

interface ResizeModalProps {
  isOpen: boolean;
  currentSize: { columns: number; rows: number };
  onClose: () => void;
  onApplyResize: (columns: number, rows: number) => void;
}

export const ResizeModal: React.FC<ResizeModalProps> = ({ isOpen, currentSize, onClose, onApplyResize }) => {
  const [resizeColumns, setResizeColumns] = React.useState(currentSize.columns);
  const [resizeRows, setResizeRows] = React.useState(currentSize.rows);

  // 当currentSize变化时，更新resizeColumns和resizeRows
  React.useEffect(() => {
    if (isOpen) {
      setResizeColumns(currentSize.columns);
      setResizeRows(currentSize.rows);
    }
  }, [isOpen, currentSize]);

  const handleApplyResize = () => {
    onApplyResize(resizeColumns, resizeRows);
    onClose();
  };

  const handleReset = () => {
    setResizeColumns(80);
    setResizeRows(24);
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50">
      <div className="bg-card border border-border rounded-xl shadow-2xl w-96 p-6">
        <div className="flex items-center justify-between mb-6">
          <h3 className="text-lg font-semibold text-foreground flex items-center gap-2">
            <Monitor size={18} className="text-primary" />
            Resize Terminal
          </h3>
          <Button 
            onClick={onClose}
            variant="ghost" 
            size="sm" 
            className="h-8 w-8 p-0 hover:bg-muted"
          >
            ×
          </Button>
        </div>
        
        <div className="space-y-6">
          {/* Current Size Preview */}
          <div className="bg-muted/50 rounded-lg p-4 border border-border">
            <div className="text-sm text-muted-foreground mb-2">Current Size</div>
            <div className="text-2xl font-mono text-primary">
              {currentSize.columns} × {currentSize.rows}
            </div>
          </div>
          
          {/* New Size Controls */}
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-foreground mb-2">
                Columns (Width)
              </label>
              <div className="flex items-center gap-3">
                <input
                  type="range"
                  min="40"
                  max="200"
                  value={resizeColumns}
                  onChange={(e) => setResizeColumns(parseInt(e.target.value))}
                  className="flex-1 h-2 bg-muted rounded-lg appearance-none cursor-pointer"
                />
                <span className="font-mono text-primary bg-muted px-3 py-1 rounded-md min-w-[60px] text-center">
                  {resizeColumns}
                </span>
              </div>
            </div>
            
            <div>
              <label className="block text-sm font-medium text-foreground mb-2">
                Rows (Height)
              </label>
              <div className="flex items-center gap-3">
                <input
                  type="range"
                  min="10"
                  max="60"
                  value={resizeRows}
                  onChange={(e) => setResizeRows(parseInt(e.target.value))}
                  className="flex-1 h-2 bg-muted rounded-lg appearance-none cursor-pointer"
                />
                <span className="font-mono text-primary bg-muted px-3 py-1 rounded-md min-w-[60px] text-center">
                  {resizeRows}
                </span>
              </div>
            </div>
          </div>
          
          {/* Action Buttons */}
          <div className="flex gap-3 pt-2">
            <Button 
              onClick={onClose}
              variant="outline" 
              className="flex-1"
            >
              Cancel
            </Button>
            <Button 
                onClick={handleReset}
                variant="outline" 
                className="flex-1"
                title="Reset to default size (80×24)"
              >
                Reset
              </Button>
            <Button 
              onClick={handleApplyResize}
              className="flex-1 bg-gradient-to-r from-primary to-primary/80 hover:from-primary/90 hover:to-primary/70"
            >
              Apply Resize
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
};