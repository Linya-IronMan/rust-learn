
import { find_all_readme, get_target_file_str, write_all_in } from './utils.js';
import {target_dir, exclude_path, custom_readme_list} from './config.js';
import path from 'path';
/*
* 1. 获取指定目录下的文件列表
* 2. 对文件列表进行分类
* 2.1 README文件
* 2.2 可访问的目录
* 3. 将README文件访问路径进行存储
* 4. 对所有可访问目录执行递归*/


const readme_list = await find_all_readme(target_dir[0]);
console.log(readme_list, 'hellow');
write_all_in(readme_list , path.resolve('./README.md'));
