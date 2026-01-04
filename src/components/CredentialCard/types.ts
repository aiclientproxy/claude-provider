/**
 * 凭证卡片组件类型定义
 */

import type { CredentialDisplay } from "@proxycast/plugin-components";

export interface CredentialCardProps {
  credential: CredentialDisplay;
  onToggle: () => void;
  onDelete: () => void;
  onReset: () => void;
  onCheckHealth: () => void;
  onRefreshToken: () => void;
  onEdit: () => void;
  deleting: boolean;
  checkingHealth: boolean;
  refreshingToken: boolean;
}

export interface CardHeaderProps {
  credential: CredentialDisplay;
  onToggle: () => void;
}

export interface CardActionsProps {
  credential: CredentialDisplay;
  onDelete: () => void;
  onReset: () => void;
  onCheckHealth: () => void;
  onRefreshToken: () => void;
  onEdit: () => void;
  deleting: boolean;
  checkingHealth: boolean;
  refreshingToken: boolean;
}

export interface CardStatsProps {
  credential: CredentialDisplay;
}

/**
 * 认证类型标签配置
 */
export const AUTH_TYPE_LABELS: Record<string, string> = {
  oauth: "OAuth",
  claude_code: "Claude Code",
  console: "Console",
  setup_token: "Setup Token",
  bedrock: "Bedrock",
  ccr: "CCR",
};

/**
 * 认证类型颜色配置
 */
export const AUTH_TYPE_COLORS: Record<string, string> = {
  oauth: "blue",
  claude_code: "purple",
  console: "green",
  setup_token: "yellow",
  bedrock: "orange",
  ccr: "gray",
};
