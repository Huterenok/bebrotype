import { FC } from "react";
import Image from "next/image";

import { IText } from "../../types";

import { Card } from "shared/ui";

import styles from "./TextCard.module.scss";

export const TextCard: FC<IText> = ({ id, content, likes, title }) => {
  return (
    <Card>
      <p>#{id}</p>
      <h1>{title}</h1>
      <p>{content.slice(0, 20)}...</p>
      <div className={styles.likes}>
        <p>{likes}</p>
        <Image width={12} height={12} src="/icons/like.svg" alt="Like" />
      </div>
    </Card>
  );
};
