#!/bin/bash

echo "🔧 Como クロスプラットフォームテストスクリプト"
echo "================================================"

# カラー定義
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# ターゲットプラットフォーム
TARGETS=(
    "x86_64-apple-darwin:macOS"
    "x86_64-pc-windows-msvc:Windows"
    "x86_64-unknown-linux-gnu:Linux"
)

echo -e "${BLUE}📋 テスト項目:${NC}"
echo "  1. フォーマットチェック"
echo "  2. Clippyリント"
echo "  3. 単体テスト"
echo "  4. クロスプラットフォームビルド"
echo "  5. バイナリサイズ確認"
echo ""

# 1. フォーマットチェック
echo -e "${YELLOW}🎨 フォーマットチェック...${NC}"
if cargo fmt --all -- --check; then
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

# 4. クロスプラットフォームビルド
echo -e "${YELLOW}🏗️  クロスプラットフォームビルド...${NC}"
mkdir -p target/releases

for target_info in "${TARGETS[@]}"; do
    IFS=':' read -r target platform <<< "$target_info"
    echo -e "${BLUE}  ► $platform ($target)${NC}"
    
    if cargo build --release --target "$target"; then
        echo -e "${GREEN}    ✅ ビルド成功${NC}"
        
        # バイナリファイルの確認
        if [[ "$target" == *"windows"* ]]; then
            binary_path="target/$target/release/como.exe"
        else
            binary_path="target/$target/release/como"
        fi
        
        if [[ -f "$binary_path" ]]; then
            file_size=$(ls -lh "$binary_path" | awk '{print $5}')
            echo -e "${GREEN}    📁 バイナリサイズ: $file_size${NC}"
            
            # リリース用ディレクトリにコピー
            release_name="como-$platform-$(echo $target | cut -d'-' -f1)"
            if [[ "$target" == *"windows"* ]]; then
                cp "$binary_path" "target/releases/${release_name}.exe"
            else
                cp "$binary_path" "target/releases/${release_name}"
            fi
        else
            echo -e "${RED}    ❌ バイナリファイルが見つかりません${NC}"
        fi
    else
        echo -e "${RED}    ❌ ビルド失敗${NC}"
    fi
    echo
done

# 5. 結果まとめ
echo -e "${BLUE}📊 ビルド結果サマリー:${NC}"
echo "===========================================" 
ls -lh target/releases/ | grep como
echo "==========================================="

echo -e "${GREEN}🎉 すべてのテストが完了しました！${NC}"
echo -e "${BLUE}💡 生成されたバイナリは target/releases/ ディレクトリにあります${NC}"
