import { FC } from "react";

import styles from "./Title.module.scss";
import classNames from "classnames";

interface TitleProps {
  children: string;
	className?: string
}

export const Title: FC<TitleProps> = ({ children, className}) => {
  return <div className={classNames(styles.wrapper, className)}>{children}</div>;
};
