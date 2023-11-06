import { FC } from "react";
import Image from "next/image";
import Link from "next/link";

import { IText } from "../../../types";

import { Card } from "shared/ui";

import styles from "./TextItem.module.scss";

export const TextItem: FC<IText> = ({ id, content, likes, title }) => {
  return (
    <Link href={`/texts/${id}`}>
      <Card className={styles.wrapper}>
        <p>#{id}</p>
        <h1>{title}</h1>
        <div className={styles.likes}>
          <Image width={12} height={12} src="/icons/like.svg" alt="Like" />
          <p>{likes}</p>
        </div>
      </Card>
    </Link>
  );
};
