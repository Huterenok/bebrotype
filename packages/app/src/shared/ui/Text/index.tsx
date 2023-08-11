import { FC } from "react";

import styles from "./Text.module.scss";

interface TextProps {
  children: string;
}

export const Text: FC<TextProps> = ({ children }) => {
  return <div className={styles.wrapper}>{children}</div>;
};
