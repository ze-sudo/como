#!/bin/bash

set -e

# カラー定義
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔧 Como ローカルテストスクリプト${NC}"
echo "================================================"
echo -e "${BLUE}📋 テスト項目:${NC}"
echo "  1. フォーマットチェック"
echo "  2. Clippyリント"
echo "  3. 単体テスト"
echo "  4. ローカルプラットフォームビルド"
echo "  5. 基本動作テスト"
echo ""

# 1. フォーマットチェック
echo -e "${YELLOW}🎨 フォーマットチェック...${NC}"
if cargo fmt -- --check; then
    echo -e "${GREEN}✅ フォーマット: OK${NC}"
else
    echo -e "${RED}❌ フォーマット: NG${NC}"
    exit 1
fi

# 2. Clippyリント
echo -e "${YELLOW}🔍 Clippyリント...${NC}"
if cargo clippy -- -D warnings; then
    echo -e "${GREEN}✅ Clippy: OK${NC}"
else
    echo -e "${RED}❌ Clippy: NG${NC}"
    exit 1
fi

# 3. 単体テスト
echo -e "${YELLOW}🧪 単体テスト...${NC}"
if cargo test; then
    echo -e "${GREEN}✅ テスト: OK${NC}"
else
    echo -e "${RED}❌ テスト: NG${NC}"
    exit 1
fi

# 4. ローカルプラットフォームビルド
echo -e "${YELLOW}🏗️  ローカルビルド...${NC}"
current_target=$(rustc -vV | grep "host:" | cut -d' ' -f2)
echo -e "  ► ローカルプラットフォーム ($current_target)"

if cargo build --release; then
    echo -e "    ${GREEN}✅ ビルド成功${NC}"
    
    # バイナリサイズ確認
    if [ -f "target/release/como" ]; then
        size=$(ls -lh target/release/como | awk '{print $5}')
        echo -e "    📏 バイナリサイズ: $size"
    fi
else
    echo -e "    ${RED}❌ ビルド失敗${NC}"
    exit 1
fi

# 5. 基本動作テスト
echo -e "${YELLOW}🔄 基本動作テスト...${NC}"
echo -e "  ► ヘルプ表示テスト"
if ./target/release/como --help > /dev/null 2>&1; then
    echo -e "    ${GREEN}✅ ヘルプ表示: OK${NC}"
else
    echo -e "    ${RED}❌ ヘルプ表示: NG${NC}"
    exit 1
fi

echo -e "  ► バージョン表示テスト"
if ./target/release/como --version > /dev/null 2>&1; then
    echo -e "    ${GREEN}✅ バージョン表示: OK${NC}"
else
    echo -e "    ${RED}❌ バージョン表示: NG${NC}"
    exit 1
fi

echo -e "  ► 実際のCLI機能テスト"
echo -e "    • リスト表示テスト"
if ./target/release/como list > /dev/null 2>&1; then
    echo -e "      ${GREEN}✅ リスト表示: OK${NC}"
else
    echo -e "      ${RED}❌ リスト表示: NG${NC}"
    exit 1
fi

echo -e "    • 新機能（ソート）テスト"
if ./target/release/como list --id > /dev/null 2>&1; then
    echo -e "      ${GREEN}✅ ID順ソート: OK${NC}"
else
    echo -e "      ${RED}❌ ID順ソート: NG${NC}"
    exit 1
fi

if ./target/release/como list --unchecked > /dev/null 2>&1; then
    echo -e "      ${GREEN}✅ 未完了フィルタ: OK${NC}"
else
    echo -e "      ${RED}❌ 未完了フィルタ: NG${NC}"
    exit 1
fi

# サマリー
echo ""
echo -e "${GREEN}📊 テスト結果サマリー:${NC}"
echo "==========================================="
echo -e "${GREEN}✅ フォーマット: PASS${NC}"
echo -e "${GREEN}✅ Clippy: PASS${NC}"
echo -e "${GREEN}✅ 単体テスト: PASS${NC}"
echo -e "${GREEN}✅ ローカルビルド: PASS${NC}"
echo -e "${GREEN}✅ 基本動作テスト: PASS${NC}"
echo -e "${GREEN}✅ 新機能テスト: PASS${NC}"
echo "==========================================="
echo -e "${GREEN}🎉 すべてのテストが完了しました！${NC}"
echo -e "${BLUE}💡 生成されたバイナリ: target/release/como${NC}"

# 追加情報を表示
echo ""
echo -e "${BLUE}📝 クロスプラットフォーム開発について:${NC}"
echo "  • Windows向けビルド: cargo-xwin や cross を使用"
echo "  • Linux向けビルド: Docker や cross を使用"
echo "  • 詳細: https://rust-lang.github.io/rustup/cross-compilation.html"
echo ""
echo -e "${BLUE}📋 追加で実行可能なテスト:${NC}"
echo "  • ./target/release/como add 'テストタスク' (タスク追加)"
echo "  • ./target/release/como list (リスト表示)"
echo "  • ./target/release/como list --id (ID順表示)"
echo "  • ./target/release/como list --unchecked (未完了のみ)"
echo "  • ./target/release/como --help (全機能確認)"
