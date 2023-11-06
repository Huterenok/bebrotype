import { invoke } from "@withease/factories";
import { createModal } from "shared/config";

export const { $isModalOpened, modalToggleEv } = invoke(createModal);
