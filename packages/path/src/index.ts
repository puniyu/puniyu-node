import path from "path";
import { APP_NAME } from "@puniyu/utils";


/** 应用插件目录 `plugins` */
export const PathPlugins = Object.freeze(path.join(process.cwd(), 'plugins').replace(/\\/g, '/'))

/** 应用主要目录 `@puniyu` */
export const PathBase = Object.freeze(
  path.join(process.cwd(), `@${APP_NAME}`).replace(/\\/g, "/")
);

/** 应用日志目录 `@puniyu/logs` */
export const PathLogs = Object.freeze(path.join(PathBase, 'logs'))

/** 应用配置目录 `@puniyu/config` */
export const PathConfig = Object.freeze(path.join(PathBase, 'config'))

/** data目录 `@puniyu/data` */
export const puniPathData = Object.freeze(path.join(PathBase, 'data'))

/** 应用资源目录 `@puniyu/resource` */
export const PathResource = Object.freeze(path.join(PathBase, 'resource'))