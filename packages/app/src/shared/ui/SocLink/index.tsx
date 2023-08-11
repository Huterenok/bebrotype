import { FC } from "react";
import Image from "next/image";

import styles from "./SocLink.module.scss";

export interface ISocLink {
  img: string;
  href: string;
  alt: string;
}

export const SocLink: FC<ISocLink> = ({ img, href, alt }) => {
  return (
    <a href={href} target="_blank" className={styles.wrapper}>
      <Image src={img} alt={alt} width={32} height={32} />
    </a>
  );
};
