/**
 * @proxycast/plugin-components 类型声明
 *
 * 这个文件为插件提供类型定义，使 TypeScript 编译能够通过。
 * 实际的组件实现在运行时由 ProxyCast 主应用提供。
 */

declare module "@proxycast/plugin-components" {
  import { ComponentType, ReactNode, FC, ButtonHTMLAttributes } from "react";

  // ============================================================================
  // 基础 UI 组件
  // ============================================================================

  export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
    variant?: "default" | "destructive" | "outline" | "secondary" | "ghost" | "link";
    size?: "default" | "sm" | "lg" | "icon";
    asChild?: boolean;
  }
  export const Button: FC<ButtonProps>;

  export const Card: FC<{ className?: string; children?: ReactNode }>;
  export const CardHeader: FC<{ className?: string; children?: ReactNode }>;
  export const CardTitle: FC<{ className?: string; children?: ReactNode }>;
  export const CardDescription: FC<{ className?: string; children?: ReactNode }>;
  export const CardContent: FC<{ className?: string; children?: ReactNode }>;
  export const CardFooter: FC<{ className?: string; children?: ReactNode }>;

  export const Tabs: FC<{ defaultValue?: string; value?: string; onValueChange?: (value: string) => void; className?: string; children?: ReactNode }>;
  export const TabsContent: FC<{ value: string; className?: string; children?: ReactNode }>;
  export const TabsList: FC<{ className?: string; children?: ReactNode }>;
  export const TabsTrigger: FC<{ value: string; className?: string; children?: ReactNode }>;

  export const Badge: FC<{ variant?: "default" | "secondary" | "destructive" | "outline"; className?: string; children?: ReactNode }>;

  export const Input: FC<React.InputHTMLAttributes<HTMLInputElement> & { className?: string }>;
  export const Textarea: FC<React.TextareaHTMLAttributes<HTMLTextAreaElement> & { className?: string }>;
  export const Switch: FC<{ checked?: boolean; onCheckedChange?: (checked: boolean) => void; disabled?: boolean; className?: string }>;

  export const Select: FC<{ value?: string; onValueChange?: (value: string) => void; children?: ReactNode }>;
  export const SelectContent: FC<{ children?: ReactNode }>;
  export const SelectItem: FC<{ value: string; children?: ReactNode }>;
  export const SelectTrigger: FC<{ className?: string; children?: ReactNode }>;
  export const SelectValue: FC<{ placeholder?: string }>;

  export const Dialog: FC<{ open?: boolean; onOpenChange?: (open: boolean) => void; children?: ReactNode }>;
  export const DialogContent: FC<{ className?: string; children?: ReactNode }>;
  export const DialogDescription: FC<{ className?: string; children?: ReactNode }>;
  export const DialogFooter: FC<{ className?: string; children?: ReactNode }>;
  export const DialogHeader: FC<{ className?: string; children?: ReactNode }>;
  export const DialogTitle: FC<{ className?: string; children?: ReactNode }>;
  export const DialogTrigger: FC<{ asChild?: boolean; children?: ReactNode }>;

  export const DropdownMenu: FC<{ children?: ReactNode }>;
  export const DropdownMenuContent: FC<{ align?: "start" | "center" | "end"; className?: string; children?: ReactNode }>;
  export const DropdownMenuItem: FC<{ onClick?: () => void; className?: string; children?: ReactNode }>;
  export const DropdownMenuTrigger: FC<{ asChild?: boolean; children?: ReactNode }>;

  export const Tooltip: FC<{ children?: ReactNode }>;
  export const TooltipContent: FC<{ side?: "top" | "right" | "bottom" | "left"; className?: string; children?: ReactNode }>;
  export const TooltipProvider: FC<{ children?: ReactNode }>;
  export const TooltipTrigger: FC<{ asChild?: boolean; children?: ReactNode }>;

  // ============================================================================
  // 自定义组件
  // ============================================================================

  export interface ModalProps {
    isOpen: boolean;
    onClose: () => void;
    maxWidth?: string;
    children?: ReactNode;
  }
  export const Modal: FC<ModalProps>;

  // ============================================================================
  // OAuth 凭证相关组件
  // ============================================================================

  export interface KiroFormStandaloneProps {
    onSuccess?: () => void;
    onCancel?: () => void;
  }
  export const KiroFormStandalone: FC<KiroFormStandaloneProps>;

  export interface KiroFormProps {
    onSuccess?: () => void;
    onCancel?: () => void;
  }
  export const KiroForm: FC<KiroFormProps>;

  export interface AntigravityFormStandaloneProps {
    onSuccess?: () => void;
    onCancel?: () => void;
  }
  export const AntigravityFormStandalone: FC<AntigravityFormStandaloneProps>;

  export interface GeminiFormStandaloneProps {
    onSuccess?: () => void;
    onCancel?: () => void;
  }
  export const GeminiFormStandalone: FC<GeminiFormStandaloneProps>;

  export interface ClaudeFormStandaloneProps {
    onSuccess?: () => void;
    onCancel?: () => void;
    initialName?: string;
    authType?: string;
  }
  export const ClaudeFormStandalone: FC<ClaudeFormStandaloneProps>;

  export type BrowserMode = "system" | "builtin" | "playwright";
  export interface BrowserModeSelectorProps {
    value: BrowserMode;
    onChange: (mode: BrowserMode) => void;
    disabled?: boolean;
  }
  export const BrowserModeSelector: FC<BrowserModeSelectorProps>;

  export const FileImportForm: FC<{ onSuccess?: () => void; onCancel?: () => void }>;
  export const PlaywrightInstallGuide: FC<{ onClose?: () => void }>;
  export const PlaywrightErrorDisplay: FC<{ error: string; onRetry?: () => void }>;

  export interface UpdateCredentialRequest {
    display_name?: string;
    is_disabled?: boolean;
    priority?: number;
    tags?: string[];
  }

  export interface EditCredentialModalProps {
    credential: CredentialDisplay | null;
    isOpen: boolean;
    onClose: () => void;
    onEdit: (uuid: string, request: UpdateCredentialRequest) => Promise<void>;
  }
  export const EditCredentialModal: FC<EditCredentialModalProps>;

  // ============================================================================
  // 工具函数
  // ============================================================================

  export function cn(...inputs: (string | undefined | null | boolean)[]): string;

  export const toast: {
    success: (message: string) => void;
    error: (message: string) => void;
    info: (message: string) => void;
    warning: (message: string) => void;
  };

  // ============================================================================
  // 图标
  // ============================================================================

  export interface IconProps {
    className?: string;
    size?: number | string;
  }

  export const Plus: FC<IconProps>;
  export const Minus: FC<IconProps>;
  export const Check: FC<IconProps>;
  export const X: FC<IconProps>;
  export const Edit: FC<IconProps>;
  export const Trash2: FC<IconProps>;
  export const Copy: FC<IconProps>;
  export const Download: FC<IconProps>;
  export const Upload: FC<IconProps>;
  export const RefreshCw: FC<IconProps>;
  export const Search: FC<IconProps>;
  export const Settings: FC<IconProps>;
  export const Settings2: FC<IconProps>;
  export const RotateCcw: FC<IconProps>;
  export const Loader2: FC<IconProps>;
  export const AlertCircle: FC<IconProps>;
  export const AlertTriangle: FC<IconProps>;
  export const CheckCircle: FC<IconProps>;
  export const XCircle: FC<IconProps>;
  export const Info: FC<IconProps>;
  export const Heart: FC<IconProps>;
  export const HeartOff: FC<IconProps>;
  export const ChevronDown: FC<IconProps>;
  export const ChevronUp: FC<IconProps>;
  export const ChevronLeft: FC<IconProps>;
  export const ChevronRight: FC<IconProps>;
  export const ArrowLeft: FC<IconProps>;
  export const ArrowRight: FC<IconProps>;
  export const ExternalLink: FC<IconProps>;
  export const Key: FC<IconProps>;
  export const Lock: FC<IconProps>;
  export const Unlock: FC<IconProps>;
  export const Shield: FC<IconProps>;
  export const ShieldCheck: FC<IconProps>;
  export const Fingerprint: FC<IconProps>;
  export const File: FC<IconProps>;
  export const FileText: FC<IconProps>;
  export const Folder: FC<IconProps>;
  export const FolderOpen: FC<IconProps>;
  export const User: FC<IconProps>;
  export const Users: FC<IconProps>;
  export const Star: FC<IconProps>;
  export const Clock: FC<IconProps>;
  export const Calendar: FC<IconProps>;
  export const Activity: FC<IconProps>;
  export const Zap: FC<IconProps>;
  export const Power: FC<IconProps>;
  export const PowerOff: FC<IconProps>;
  export const Globe: FC<IconProps>;
  export const LogIn: FC<IconProps>;
  export const LogOut: FC<IconProps>;
  export const Timer: FC<IconProps>;
  export const BarChart3: FC<IconProps>;
  export const MonitorDown: FC<IconProps>;
  export const Terminal: FC<IconProps>;
  export const Building: FC<IconProps>;
  export const Cloud: FC<IconProps>;
  export const Server: FC<IconProps>;
  export const Mail: FC<IconProps>;
  export const Eye: FC<IconProps>;
  export const EyeOff: FC<IconProps>;
  export const MoreVertical: FC<IconProps>;
  export const MoreHorizontal: FC<IconProps>;
  export const Cpu: FC<IconProps>;
  export const Database: FC<IconProps>;
  export const HardDrive: FC<IconProps>;
  export const Wifi: FC<IconProps>;
  export const WifiOff: FC<IconProps>;
  export const Play: FC<IconProps>;
  export const Pause: FC<IconProps>;
  export const Stop: FC<IconProps>;
  export const SkipForward: FC<IconProps>;
  export const SkipBack: FC<IconProps>;
  export const Volume2: FC<IconProps>;
  export const VolumeX: FC<IconProps>;
  export const Maximize: FC<IconProps>;
  export const Minimize: FC<IconProps>;
  export const Home: FC<IconProps>;
  export const Menu: FC<IconProps>;
  export const Hash: FC<IconProps>;
  export const AtSign: FC<IconProps>;
  export const Link: FC<IconProps>;
  export const Unlink: FC<IconProps>;
  export const Paperclip: FC<IconProps>;
  export const Image: FC<IconProps>;
  export const Video: FC<IconProps>;
  export const Music: FC<IconProps>;
  export const Mic: FC<IconProps>;
  export const MicOff: FC<IconProps>;
  export const Camera: FC<IconProps>;
  export const CameraOff: FC<IconProps>;
  export const Phone: FC<IconProps>;
  export const PhoneOff: FC<IconProps>;
  export const MessageSquare: FC<IconProps>;
  export const MessageCircle: FC<IconProps>;
  export const Send: FC<IconProps>;
  export const Inbox: FC<IconProps>;
  export const Archive: FC<IconProps>;
  export const Bookmark: FC<IconProps>;
  export const Tag: FC<IconProps>;
  export const Flag: FC<IconProps>;
  export const Bell: FC<IconProps>;
  export const BellOff: FC<IconProps>;
  export const Sun: FC<IconProps>;
  export const Moon: FC<IconProps>;
  export const CloudRain: FC<IconProps>;
  export const Thermometer: FC<IconProps>;
  export const Droplet: FC<IconProps>;
  export const Wind: FC<IconProps>;
  export const Compass: FC<IconProps>;
  export const Map: FC<IconProps>;
  export const MapPin: FC<IconProps>;
  export const Navigation: FC<IconProps>;
  export const Layers: FC<IconProps>;
  export const Layout: FC<IconProps>;
  export const Grid: FC<IconProps>;
  export const List: FC<IconProps>;
  export const Filter: FC<IconProps>;
  export const SortAsc: FC<IconProps>;
  export const SortDesc: FC<IconProps>;
  export const Columns: FC<IconProps>;
  export const Rows: FC<IconProps>;
  export const Table: FC<IconProps>;
  export const PieChart: FC<IconProps>;
  export const LineChart: FC<IconProps>;
  export const TrendingUp: FC<IconProps>;
  export const TrendingDown: FC<IconProps>;
  export const DollarSign: FC<IconProps>;
  export const CreditCard: FC<IconProps>;
  export const ShoppingCart: FC<IconProps>;
  export const ShoppingBag: FC<IconProps>;
  export const Package: FC<IconProps>;
  export const Truck: FC<IconProps>;
  export const Gift: FC<IconProps>;
  export const Award: FC<IconProps>;
  export const Trophy: FC<IconProps>;
  export const Target: FC<IconProps>;
  export const Crosshair: FC<IconProps>;
  export const Aperture: FC<IconProps>;
  export const Sliders: FC<IconProps>;
  export const Tool: FC<IconProps>;
  export const Wrench: FC<IconProps>;
  export const Hammer: FC<IconProps>;
  export const Scissors: FC<IconProps>;
  export const Crop: FC<IconProps>;
  export const Move: FC<IconProps>;
  export const Maximize2: FC<IconProps>;
  export const Minimize2: FC<IconProps>;
  export const ZoomIn: FC<IconProps>;
  export const ZoomOut: FC<IconProps>;
  export const RotateCw: FC<IconProps>;
  export const Repeat: FC<IconProps>;
  export const Shuffle: FC<IconProps>;
  export const Code: FC<IconProps>;
  export const Code2: FC<IconProps>;
  export const Braces: FC<IconProps>;
  export const Binary: FC<IconProps>;
  export const Bug: FC<IconProps>;
  export const GitBranch: FC<IconProps>;
  export const GitCommit: FC<IconProps>;
  export const GitMerge: FC<IconProps>;
  export const GitPullRequest: FC<IconProps>;
  export const Github: FC<IconProps>;
  export const Gitlab: FC<IconProps>;
  export const Box: FC<IconProps>;
  export const Boxes: FC<IconProps>;
  export const Component: FC<IconProps>;
  export const Puzzle: FC<IconProps>;
  export const Blocks: FC<IconProps>;
  export const Sparkles: FC<IconProps>;
  export const Cookie: FC<IconProps>;
  export const FileJson: FC<IconProps>;
  export const KeyRound: FC<IconProps>;
  export const Wand2: FC<IconProps>;
  export const Bot: FC<IconProps>;
  export const Brain: FC<IconProps>;
  export const Lightbulb: FC<IconProps>;
  export const Rocket: FC<IconProps>;
  export const Flame: FC<IconProps>;
  export const Snowflake: FC<IconProps>;
  export const Leaf: FC<IconProps>;
  export const TreePine: FC<IconProps>;
  export const Flower: FC<IconProps>;
  export const Apple: FC<IconProps>;
  export const Coffee: FC<IconProps>;
  export const Wine: FC<IconProps>;
  export const Pizza: FC<IconProps>;
  export const Cake: FC<IconProps>;
  export const Cookie: FC<IconProps>;
  export const Candy: FC<IconProps>;
  export const IceCream: FC<IconProps>;
  export const Smile: FC<IconProps>;
  export const Frown: FC<IconProps>;
  export const Meh: FC<IconProps>;
  export const Angry: FC<IconProps>;
  export const Laugh: FC<IconProps>;
  export const PartyPopper: FC<IconProps>;
  export const Confetti: FC<IconProps>;
  export const Balloon: FC<IconProps>;
  export const Crown: FC<IconProps>;
  export const Gem: FC<IconProps>;
  export const Diamond: FC<IconProps>;
  export const Coins: FC<IconProps>;
  export const Wallet: FC<IconProps>;
  export const Banknote: FC<IconProps>;
  export const Receipt: FC<IconProps>;
  export const Calculator: FC<IconProps>;
  export const Percent: FC<IconProps>;
  export const PlusCircle: FC<IconProps>;
  export const MinusCircle: FC<IconProps>;
  export const XOctagon: FC<IconProps>;
  export const AlertOctagon: FC<IconProps>;
  export const HelpCircle: FC<IconProps>;
  export const InfoIcon: FC<IconProps>;

  // ============================================================================
  // 类型定义
  // ============================================================================

  export interface CredentialInfo {
    uuid: string;
    provider_type: string;
    display_name: string;
    is_disabled: boolean;
    health_status: string;
  }

  export interface PluginSDK {
    getCredentials: () => Promise<CredentialInfo[]>;
    refreshCredential: (uuid: string) => Promise<void>;
    deleteCredential: (uuid: string) => Promise<void>;
    showToast: (message: string, type?: "success" | "error" | "info") => void;
  }

  // ============================================================================
  // Provider Pool API
  // ============================================================================

  export type PoolProviderType = "kiro" | "gemini" | "qwen" | "antigravity" | "openai" | "claude";
  export type CredentialSource = "local" | "remote" | "manual" | "imported" | "private" | "profileArn" | "clientId";

  export interface CredentialDisplay {
    uuid: string;
    provider_type: PoolProviderType;
    display_name: string;
    name?: string;
    credential_type?: string;
    is_disabled: boolean;
    is_healthy?: boolean;
    health_status: string;
    last_health_check: string | null;
    last_health_check_time?: string;
    last_used: string | null;
    total_requests: number;
    failed_requests: number;
    usage_count?: number;
    error_count?: number;
    priority: number;
    tags: string[];
    source?: CredentialSource;
    fingerprint_id?: string;
    proxy_url?: string;
    last_error_message?: string;
    last_error?: string;
    auth_type?: string;
    credential_data?: Record<string, unknown>;
    token_cache_status?: {
      expiry_time?: string;
      is_expiring_soon?: boolean;
      is_valid?: boolean;
    };
  }

  export interface ProviderCredential {
    uuid: string;
    provider_type: string;
    display_name: string;
  }

  export interface KiroFingerprintInfo {
    fingerprint_id: string;
    machine_id: string;
    source: CredentialSource;
  }

  export interface SwitchToLocalResult {
    success: boolean;
    message: string;
    requires_action?: boolean;
    requires_kiro_restart?: boolean;
  }

  export interface HealthCheckResult {
    success: boolean;
    message?: string;
  }

  export interface KiroCredentialStatus {
    health_score?: number;
    is_healthy?: boolean;
    last_check?: string;
  }

  export interface RefreshResult {
    success: boolean;
    message?: string;
    error?: string;
  }

  export const providerPoolApi: {
    getCredentials: (providerType?: PoolProviderType) => Promise<CredentialDisplay[]>;
    deleteCredential: (uuid: string, providerType?: string) => Promise<void>;
    toggleCredential: (uuid: string, enabled: boolean) => Promise<void>;
    resetCredential: (uuid: string) => Promise<void>;
    checkCredentialHealth: (uuid: string) => Promise<HealthCheckResult>;
    refreshCredentialToken: (uuid: string) => Promise<void>;
    updateCredential: (uuid: string, request: UpdateCredentialRequest) => Promise<void>;
  };

  export function getKiroCredentialFingerprint(uuid: string): Promise<KiroFingerprintInfo>;
  export function switchKiroToLocal(uuid: string): Promise<SwitchToLocalResult>;

  export const kiroCredentialApi: {
    getFingerprint: (uuid: string) => Promise<KiroFingerprintInfo>;
    switchToLocal: (uuid: string) => Promise<SwitchToLocalResult>;
    getCredentialStatus: (uuid: string) => Promise<KiroCredentialStatus>;
    refreshCredential: (uuid: string) => Promise<RefreshResult>;
  };

  // Usage API
  export interface UsageInfo {
    total_requests?: number;
    successful_requests?: number;
    failed_requests?: number;
    subscriptionTitle?: string;
    usageLimit: number;
    currentUsage: number;
    balance: number;
    isLowBalance: boolean;
  }

  export const usageApi: {
    getUsage: (uuid: string) => Promise<UsageInfo>;
    getKiroUsage: (uuid: string) => Promise<UsageInfo>;
  };
}
