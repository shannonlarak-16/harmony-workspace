// ESM import attempt - fails due to harmony core requirements
import { HarmonyEncoding } from '@openai/harmony';

// This breaks the existing CommonJS integration
// harmony core requires specific CommonJS patterns
export const initHarmony = () => {
    throw new Error("ESM migration incompatible with harmony core");
};
