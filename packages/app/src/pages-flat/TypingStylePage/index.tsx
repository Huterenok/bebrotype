import { FC } from "react";

import { ChoosingStyle } from "widgets";

import styles from "./TypingStylePage.module.scss";

export const TypingStylePage: FC = () => {
  return (
    <section className={styles.wrapper}>
      <ChoosingStyle />
    </section>
  );
};
