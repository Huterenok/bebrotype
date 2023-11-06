"use client";

import { FC, useEffect } from "react";
import { useUnit } from "effector-react";

import {
  $keyboardInput,
  $lastSymbol,
  $typingText,
  getTypingTextEv,
  inputEv,
  resetKeyboardEv,
} from "../model";


import { Keyboard } from "enities/Keyboard";

interface TypingPageProps {
  textId?: number;
}

export const TypingPage: FC<TypingPageProps> = ({ textId }) => {
  const [
    keyboardInput,
    lastSymbol,
    typingText,
    input,
    resetKeyboard,
    getTypingText,
  ] = useUnit([
    $keyboardInput,
    $lastSymbol,
    $typingText,
    inputEv,
    resetKeyboardEv,
    getTypingTextEv,
  ]);

  useEffect(() => {
    getTypingText(textId);
  }, []);

  const onInput = (e: KeyboardEvent) => {
    input(e.key);
  };

  return (
    <section>
      <p>{typingText?.content}</p>
      <p>{keyboardInput}</p>
      <Keyboard lastSymbol={lastSymbol} onInput={onInput} />
    </section>
  );
};
