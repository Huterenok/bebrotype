import { FC } from "react";
import Link from "next/link";
import Image from "next/image";

import { Card } from "shared/ui/Card";

import { ITypingCard } from "../types";
import styles from "./TypingCard.module.scss";

export const TypingCard: FC<ITypingCard> = ({
  icon,
  subtitle,
  title,
  href,
}) => {
  return (
    <Link href={href}>
      <Card>
        <div className={styles.container}>
          <Image
            alt={title}
            src={icon}
            width={48}
            height={48}
            className={styles.icon}
          />
          <h1 className={styles.title}>{title}</h1>
          <p className={styles.subtitle}>{subtitle}</p>
        </div>
      </Card>
    </Link>
  );
};
