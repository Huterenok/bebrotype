import { FC } from "react";

import { ChoosingStyle } from "widgets";

import styles from "./Typing.module.scss";

export const TypingPage: FC = () => {
  return (
    <section className={styles.wrapper}>
      <ChoosingStyle />
    </section>
  );
};
