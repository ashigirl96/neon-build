// @ts-ignore
import Module from './index.node'

export const hello: () => string = Module.hello

export class UserDict {
  // constructor() {
  //   Module.userDictConstructor()
  // }
  //
  // public addEntry(word: string, yomi: string, pos: string): void {
  //   Module.userDictAddEntry(word, yomi, pos)
  // }
  //
  // public save(path: string): void {
  //   Module.userDictSave(path)
  // }
}

export class OpenJTalk {
  constructor(openJTalkDictDir: string) {
    return Module.openjtalkConstructor(openJTalkDictDir)
  }

  public useUserDict(userDict: UserDict) {
    Module.openjtalkUseUserDict(userDict)
  }
}

export class Synthesizer {
  constructor(openJTalk: OpenJTalk, accelerationMode: "Auto" |"Cpu" | "Gpu", cpuNumThreads: number) {
    return Module.synthesizerConstructor(openJTalk, accelerationMode, cpuNumThreads)
  }
}

const openJTalk = new OpenJTalk('/Users/nishimura/.ghq/src/github.com/ashigirl96/voicevox_core_node_api/download/voicevox_core/open_jtalk_dic_utf_8-1.11')
const synthesizer = new Synthesizer(openJTalk, "Auto", 1)
console.log(synthesizer)