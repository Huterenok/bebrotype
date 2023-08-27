import { FC } from "react";
import Image from "next/image";
import Link from "next/link";

import { INavLink } from "../../types";

import styles from "./NavbarLink.module.scss";

export const NavbarLink: FC<INavLink> = ({ img, subtitle, to }) => {
  return (
    <Link href={to} className={styles.navLink}>
      <Image alt={subtitle} src={img} width={32} height={32} />
      <p>{subtitle}</p>
    </Link>
  );
};
