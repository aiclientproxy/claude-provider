/**
 * 凭证卡片组件
 * 显示单个 Claude 凭证的信息和操作按钮
 */

import {
  Button,
  Badge,
  Card,
  CardHeader,
  CardContent,
  CardFooter,
  Switch,
  Loader2,
  Trash2,
  RefreshCw,
  Zap,
  Edit,
  RotateCcw,
  Key,
  Terminal,
  Building,
  Lock,
  Cloud,
  Server,
  CheckCircle,
  XCircle,
  Clock,
  Mail,
  Globe,
} from "@proxycast/plugin-components";
import type { CredentialCardProps } from "./types";
import { AUTH_TYPE_LABELS, AUTH_TYPE_COLORS } from "./types";

interface CredentialData {
  auth_type?: string;
  email?: string;
  region?: string;
  base_url?: string;
  expire?: string;
  last_refresh?: string;
}

function getAuthTypeIcon(authType: string) {
  switch (authType) {
    case "oauth": return Key;
    case "claude_code": return Terminal;
    case "console": return Building;
    case "setup_token": return Lock;
    case "bedrock": return Cloud;
    case "ccr": return Server;
    default: return Key;
  }
}

function formatDate(dateStr: string): string {
  try {
    return new Date(dateStr).toLocaleString("zh-CN", {
      month: "2-digit", day: "2-digit", hour: "2-digit", minute: "2-digit",
    });
  } catch { return dateStr; }
}

function supportsTokenRefresh(authType: string): boolean {
  return ["oauth", "claude_code", "console"].includes(authType);
}

export function CredentialCard({
  credential, onToggle, onDelete, onReset, onCheckHealth, onRefreshToken, onEdit,
  deleting, checkingHealth, refreshingToken,
}: CredentialCardProps) {
  const data = (credential.credential_data || {}) as CredentialData;
  const authType = data.auth_type || "oauth";
  const isHealthy = !credential.is_disabled && credential.health_status !== "unhealthy";
  const AuthIcon = getAuthTypeIcon(authType);

  return (
    <Card className={credential.is_disabled ? "opacity-60" : ""}>
      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
        <div className="flex items-center gap-3">
          <div className="flex items-center gap-2">
            {isHealthy ? <CheckCircle className="h-5 w-5 text-green-500" /> : <XCircle className="h-5 w-5 text-red-500" />}
          </div>
          <div>
            <h3 className="font-medium">{credential.name || "未命名凭证"}</h3>
            <p className="text-sm text-muted-foreground">{credential.uuid.slice(0, 8)}...</p>
          </div>
          <Badge variant="outline" className={`bg-${AUTH_TYPE_COLORS[authType] || "gray"}-50`}>
            <AuthIcon className="h-3 w-3 mr-1" />
            {AUTH_TYPE_LABELS[authType] || authType}
          </Badge>
        </div>
        <Switch checked={!credential.is_disabled} onCheckedChange={onToggle} />
      </CardHeader>

      <CardContent className="space-y-3">
        <div className="grid grid-cols-2 gap-4 text-sm">
          {data.email && (
            <div className="flex items-center gap-2">
              <Mail className="h-4 w-4 text-muted-foreground" />
              <span className="text-muted-foreground">邮箱:</span>
              <span>{String(data.email)}</span>
            </div>
          )}
          {data.region && (
            <div className="flex items-center gap-2">
              <Globe className="h-4 w-4 text-muted-foreground" />
              <span className="text-muted-foreground">区域:</span>
              <span>{String(data.region)}</span>
            </div>
          )}
          {data.base_url && (
            <div className="flex items-center gap-2 col-span-2">
              <Globe className="h-4 w-4 text-muted-foreground" />
              <span className="text-muted-foreground">Base URL:</span>
              <span className="truncate">{String(data.base_url)}</span>
            </div>
          )}
          {data.expire && (
            <div className="flex items-center gap-2">
              <Clock className="h-4 w-4 text-muted-foreground" />
              <span className="text-muted-foreground">过期:</span>
              <span>{formatDate(String(data.expire))}</span>
            </div>
          )}
          {data.last_refresh && (
            <div className="flex items-center gap-2">
              <RefreshCw className="h-4 w-4 text-muted-foreground" />
              <span className="text-muted-foreground">刷新:</span>
              <span>{formatDate(String(data.last_refresh))}</span>
            </div>
          )}
        </div>
        <div className="flex items-center gap-4 text-sm text-muted-foreground">
          <span>使用: {credential.usage_count || 0} 次</span>
          <span>错误: {credential.error_count || 0} 次</span>
          {credential.last_error && <span className="text-red-500 truncate">最后错误: {credential.last_error}</span>}
        </div>
      </CardContent>

      <CardFooter className="flex justify-end gap-2">
        <Button variant="ghost" size="sm" onClick={onEdit}><Edit className="h-4 w-4" /></Button>
        <Button variant="ghost" size="sm" onClick={onReset}><RotateCcw className="h-4 w-4" /></Button>
        <Button variant="ghost" size="sm" onClick={onCheckHealth} disabled={checkingHealth}>
          {checkingHealth ? <Loader2 className="h-4 w-4 animate-spin" /> : <Zap className="h-4 w-4" />}
        </Button>
        {supportsTokenRefresh(authType) && (
          <Button variant="ghost" size="sm" onClick={onRefreshToken} disabled={refreshingToken}>
            {refreshingToken ? <Loader2 className="h-4 w-4 animate-spin" /> : <RefreshCw className="h-4 w-4" />}
          </Button>
        )}
        <Button variant="ghost" size="sm" onClick={onDelete} disabled={deleting} className="text-red-500 hover:text-red-600">
          {deleting ? <Loader2 className="h-4 w-4 animate-spin" /> : <Trash2 className="h-4 w-4" />}
        </Button>
      </CardFooter>
    </Card>
  );
}