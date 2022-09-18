
import path from 'path';
import fs from "fs";

// const path = require('path');
// const fs = require('fs');

export const isDir = (target) => {
    return exists(target) && fs.statSync(target).isDirectory();
}

export const exists = (target) => {
     return fs.existsSync(target) || path.existsSync(target);
}

export const find_all_readme = (p_root) => {
  const readme_path_list = [];
  const _fn = (p) => {
    const curdir_filelist = fs.readdirSync(p);
    if (curdir_filelist.includes('README.md')) {
      readme_path_list.push(`${p}/README.md`.replace('//', '/'));
    }
    curdir_filelist.forEach(_ => {
      const target_path = path.resolve(`${p}/${_}`.replace('//', '/'));
      const isdir = isDir(target_path);
      const isThirdPartyFile = target_path.includes('node_modules');
      if (isdir && !isThirdPartyFile ) _fn(target_path);
    })
  }
  console.log(p_root, 'p_root ===>') ;
  _fn(p_root);
  return readme_path_list;
}

export const get_target_file_str = (file_path) => {
  return  fs.readFileSync(file_path, { encoding: 'utf8' });
}

export const write_all_in = (file_list, target_file) => {
  file_list.forEach(file_path => {
    const content = get_target_file_str(file_path);
    fs.writeFileSync(target_file, content, { flag: 'a' });
  })
}
