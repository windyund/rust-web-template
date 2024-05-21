#!/usr/bin/env bash
# -*- coding: utf-8 -*-
# 默认参数值
RUST_RELEASE_MODE=debug
APP_NAME=rust-web

# 使用示例
# 启动服务，使用默认的debug模式和应用名称rust-web
# ./run.sh start

# 停止服务，使用release模式和自定义的应用名称custom_app
# ./run.sh -m release -n custom_app stop

# 读取传入的参数
while [[ $# -gt 0 ]]; do
    key="$1"
    case $key in
        -m|--mode)
        RUST_RELEASE_MODE="$2"
        shift
        shift
        ;;
        -n|--name)
        APP_NAME="$2"
        shift
        shift
        ;;
        start|stop|restart)
        ACTION="$1"
        shift
        ;;
        *)
        echo "Unknown option: $1"
        exit 1
        ;;
    esac
done

# 启动服务函数
start_service() {
    echo "Starting service..."
    # 清理和构建
#    cargo fmt
#    cargo clean
    if [ "$RUST_RELEASE_MODE" = "release" ]; then
       cargo build --release
    else
       cargo build
    fi
    nohup ./target/"$RUST_RELEASE_MODE"/"$APP_NAME" > "./${APP_NAME}.log" 2>&1 &
    echo "Service started. Logging to ./${APP_NAME}.log"
}

# 停止服务函数
stop_service() {
    echo "Stopping service..."
    PID=$(pgrep -f "./$APP_NAME")
    if [ ! -z "$PID" ]; then
        kill $PID
        echo "Service stopped."
    else
        echo "No running service found."
    fi
}

# 根据传入的参数执行不同的操作
case $ACTION in
    start)
        start_service
        ;;
    stop)
        stop_service
        ;;
    restart)
        echo "Restarting service..."
        stop_service
        sleep 2 # 等待确保服务已完全停止
        start_service
        ;;
    *)
        echo "Unknown action: $ACTION"
        exit 1
        ;;
esac
