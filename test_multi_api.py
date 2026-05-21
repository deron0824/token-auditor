#!/usr/bin/env python3
"""
TokenAuditor 多平台 API 测试脚本
支持: OpenAI, 阿里百炼(DashScope), 火山引擎(Ark)
"""

import os
import sys

# 加载环境变量（从 .zshrc）
def load_env_from_zshrc():
    """从 .zshrc 加载环境变量"""
    zshrc_path = os.path.expanduser("~/.zshrc")
    if os.path.exists(zshrc_path):
        with open(zshrc_path, 'r') as f:
            for line in f:
                line = line.strip()
                if line.startswith('export ') and '=' in line:
                    # 提取变量名和值
                    var_part = line[7:]  # 去掉 'export '
                    if '=' in var_part:
                        key, value = var_part.split('=', 1)
                        key = key.strip()
                        value = value.strip().strip('"').strip("'")
                        os.environ[key] = value

# 加载环境变量
load_env_from_zshrc()

# 设置代理环境变量（如果还没设置）
if not os.getenv("HTTP_PROXY"):
    os.environ["HTTP_PROXY"] = "http://127.0.0.1:11435"
if not os.getenv("HTTPS_PROXY"):
    os.environ["HTTPS_PROXY"] = "http://127.0.0.1:11435"

print("=" * 60)
print("🔍 TokenAuditor 多平台 API 测试")
print("=" * 60)
print(f"\n📡 代理设置:")
print(f"   HTTP_PROXY: {os.getenv('HTTP_PROXY')}")
print(f"   HTTPS_PROXY: {os.getenv('HTTPS_PROXY')}\n")

# 检测可用的 API Key
available_apis = {}

if os.getenv("OPENAI_API_KEY"):
    available_apis["openai"] = os.getenv("OPENAI_API_KEY")[:10] + "..."
    print(f"✅ OpenAI API Key: {available_apis['openai']}")

if os.getenv("DASHSCOPE_API_KEY"):
    available_apis["dashscope"] = os.getenv("DASHSCOPE_API_KEY")[:10] + "..."
    print(f"✅ 阿里百炼 API Key: {available_apis['dashscope']}")

if os.getenv("ARK_API_KEY"):
    available_apis["ark"] = os.getenv("ARK_API_KEY")[:10] + "..."
    print(f"✅ 火山引擎 API Key: {available_apis['ark']}")

if not available_apis:
    print("\n❌ 未找到任何 API Key")
    print("请确保在 ~/.zshrc 中已配置以下环境变量之一：")
    print("  - OPENAI_API_KEY")
    print("  - DASHSCOPE_API_KEY")
    print("  - ARK_API_KEY")
    sys.exit(1)

print(f"\n📊 共找到 {len(available_apis)} 个 API Key\n")

# 测试函数
def test_openai():
    """测试 OpenAI API"""
    try:
        from openai import OpenAI
        
        print("📤 测试 OpenAI GPT-4o...")
        
        client = OpenAI(
            api_key=os.getenv("OPENAI_API_KEY"),
            base_url="https://api.openai.com/v1"
        )
        
        response = client.chat.completions.create(
            model="gpt-4o",
            messages=[
                {"role": "user", "content": "Hello from TokenAuditor!"}
            ],
            max_tokens=30
        )
        
        print(f"✅ 成功！")
        print(f"   模型: {response.model}")
        print(f"   内容: {response.choices[0].message.content}")
        print(f"   Token: {response.usage.prompt_tokens} in / {response.usage.completion_tokens} out / {response.usage.total_tokens} total")
        return True
        
    except Exception as e:
        print(f"❌ 失败: {e}")
        return False

def test_dashscope():
    """测试阿里百炼 DashScope API"""
    try:
        from openai import OpenAI
        
        print("\n📤 测试阿里百炼 qwen-turbo...")
        
        # 使用 OpenAI 兼容接口
        client = OpenAI(
            api_key=os.getenv("DASHSCOPE_API_KEY"),
            base_url="https://dashscope.aliyuncs.com/compatible-mode/v1"
        )
        
        response = client.chat.completions.create(
            model="qwen-turbo",
            messages=[
                {"role": "user", "content": "Hello from TokenAuditor!"}
            ],
            max_tokens=30
        )
        
        print(f"✅ 成功！")
        print(f"   模型: {response.model}")
        print(f"   内容: {response.choices[0].message.content}")
        print(f"   Token: {response.usage.prompt_tokens} in / {response.usage.completion_tokens} out / {response.usage.total_tokens} total")
        return True
        
    except Exception as e:
        print(f"❌ 失败: {e}")
        return False

def test_ark():
    """测试火山引擎 Ark API"""
    try:
        from openai import OpenAI
        
        print("\n📤 测试火山引擎 doubao-lite...")
        
        # 使用 OpenAI 兼容接口
        client = OpenAI(
            api_key=os.getenv("ARK_API_KEY"),
            base_url="https://ark.cn-beijing.volces.com/api/v3"
        )
        
        # 尝试多个可能的模型名称
        models_to_try = ["doubao-lite-32k", "doubao-lite-4k", "doubao-pro-32k"]
        
        for model in models_to_try:
            try:
                print(f"   尝试模型: {model}...")
                response = client.chat.completions.create(
                    model=model,
                    messages=[
                        {"role": "user", "content": "Hello from TokenAuditor!"}
                    ],
                    max_tokens=30
                )
                
                print(f"✅ 成功！")
                print(f"   模型: {response.model}")
                print(f"   内容: {response.choices[0].message.content}")
                print(f"   Token: {response.usage.prompt_tokens} in / {response.usage.completion_tokens} out / {response.usage.total_tokens} total")
                return True
            except Exception as e:
                if "does not exist" in str(e) or "not found" in str(e).lower():
                    print(f"   ❌ 模型不存在，尝试下一个...")
                    continue
                else:
                    raise e
        
        print(f"❌ 所有模型都失败了")
        return False
        
    except Exception as e:
        print(f"❌ 失败: {e}")
        return False

# 根据可用的 API Key 运行测试
print("=" * 60)
print("开始测试")
print("=" * 60)

results = {}

if "openai" in available_apis:
    results["OpenAI"] = test_openai()

if "dashscope" in available_apis:
    results["阿里百炼"] = test_dashscope()

if "ark" in available_apis:
    results["火山引擎"] = test_ark()

# 输出总结
print("\n" + "=" * 60)
print("📊 测试总结")
print("=" * 60)

for platform, success in results.items():
    status = "✅ 通过" if success else "❌ 失败"
    print(f"{platform:10} {status}")

success_count = sum(1 for s in results.values() if s)
total_count = len(results)

print(f"\n总计: {success_count}/{total_count} 通过")

if success_count > 0:
    print("\n🎉 测试完成！请查看 TokenAuditor 终端的输出")
else:
    print("\n⚠️  所有测试都失败了，请检查 API Key 和网络连接")

print("=" * 60)
