"use client";

import { FC, useEffect } from "react";

import { keyboardLayout } from "../config/layout";
import { Keycup } from "./Keycup";

import styles from "./Keyboard.module.scss";

interface KeyboardProps {
  lastSymbol: string;
  onInput: (e: KeyboardEvent) => void;
}

export const Keyboard: FC<KeyboardProps> = ({ lastSymbol, onInput }) => {
  useEffect(() => {
    document.addEventListener("keydown", onInput);

    return () => document.removeEventListener("keydown", onInput);
  }, [onInput]);

  return (
    <div className={styles.wrapper}>
      {keyboardLayout.map((row, i) => (
        <div className={styles.row} key={i}>
          {row.map((elem) => (
            <Keycup
              value={elem}
              isActive={elem.includes(lastSymbol)}
              key={elem}
            />
          ))}
        </div>
      ))}
    </div>
  );
};
