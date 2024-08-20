import fs from "node:fs";
import path from "node:path"; 

export namespace FileSystem {
  export const findDir = (dirName: string, srcDirPath: string = process.cwd()) => {
    let matchingDirPaths: Array<string> = [];

    const findDirRecurse = (srcDirPath: string) => {
      const subDirNames = fs.readdirSync(srcDirPath, { withFileTypes: true })
        .filter(dirEntry => dirEntry.isDirectory())
        .map(dirEntry => dirEntry.name);

      for (const subDirName of subDirNames) {
        const subDirPath = path.join(srcDirPath, subDirName);

        if (subDirName === dirName)
          matchingDirPaths.push(subDirPath);

        findDirRecurse(subDirPath);
      }
    };

    findDirRecurse(srcDirPath);
    return matchingDirPaths;
  };

  const findFileByPredicates = (options: {
    predicates: Array<(fileName: string) => boolean>;
    every: boolean;
  }, srcDirPath: string) => {
    let matchingFilePaths: Array<string> = [];

    const predicate = (fileName: string) => {
      return options.every
        ? options.predicates.every(predicate => predicate(fileName))
        : options.predicates.some(predicate => predicate(fileName));
    };

    const findFileRecurse = (srcDirPath: string) => {
      const dirEntries = fs.readdirSync(srcDirPath, { withFileTypes: true });
      
      for (const dirEntry of dirEntries) {
        const subDirOrFilePath = path.join(srcDirPath, dirEntry.name);

        if (dirEntry.isDirectory())
          findFileRecurse(subDirOrFilePath);
        else if (dirEntry.isFile() && predicate(dirEntry.name))
          matchingFilePaths.push(subDirOrFilePath);
      }
    };

    findFileRecurse(srcDirPath);
    return matchingFilePaths;
  };

  export interface Options {
    names?: Array<string>;
    exts?: Array<string>;
    basenames?: Array<string>;
    every?: boolean;
  }

  export const findFile = (options: Options, srcDirPath: string) => {
    let predicates: Array<(fileName: string) => boolean> = [];

    if (options.names)
      predicates.push(...options.names.map(
        name => ((fileName: string) => fileName === name)
      ));

    if (options.exts)
      predicates.push(...options.exts.map(
        ext => ((fileName: string) => fileName.endsWith(ext))
      ));

    if (options.basenames)
      predicates.push(...options.basenames.map(
        basename => ((fileName: string) => path.basename(fileName, path.extname(fileName)) === basename)
      ));

    return findFileByPredicates({
      predicates: predicates,
      every: options.every ?? false,
    }, srcDirPath);
  };
}