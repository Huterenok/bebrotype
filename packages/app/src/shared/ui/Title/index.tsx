import { FC } from "react";

import styles from "./Title.module.css";

interface TitleProps {
  children: string;
}

export const Title: FC<TitleProps> = ({ children }) => {
  return <div className={styles.wrapper}>{children}</div>;
};
