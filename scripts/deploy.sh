#!/bin/bash
# -*- coding: utf-8 -*-
# 拉取最新的代码
git pull origin master
# 重启服务
sh scripts/run.sh restart
