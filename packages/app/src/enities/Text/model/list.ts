// import { createEvent, createStore, sample } from "effector";

// import { IText } from "../types";
// import { createFactory } from "@withease/factories";

// interface createTextListParams {
//   initialValue?: IText[];
// }

// //TODO
// export const createTextList = createFactory(
//   ({ initialValue = [] }: createTextListParams) => {
//     const setAll = createEvent<IText[]>();
//     const reset = createEvent();

//     const $texts = createStore<IText[]>(initialValue);
//     $texts.reset(reset);

//     sample({
//       clock: setAll,
//       target: $texts,
//     });

//     return {
//       $texts,
//       setAll,
//       reset,
//     };
//   }
// );
