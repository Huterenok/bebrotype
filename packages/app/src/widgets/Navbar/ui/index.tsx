import { FC } from "react";
import Image from "next/image";
import Link from "next/link";

import { SocLink } from "shared/ui/SocLink";
import { NavbarLink } from "./NavbarLink";

import { navLinks, socLinks } from "../config/links";
import styles from "./Navbar.module.scss";

export const Navbar: FC = () => {
  return (
    <nav className={styles.wrapper}>
      <div className={styles.container}>
        <Link href={"/"}>
          <Image
            alt={"Logo"}
            src={"/icons/banana.svg"}
            width={48}
            height={48}
            className={styles.logo}
          />
        </Link>
        <div className={styles.navigation}>
          {navLinks.map(({ img, to, subtitle }) => (
            <NavbarLink img={img} subtitle={subtitle} to={to} key={to} />
          ))}
        </div>
        <div className={styles.social}>
          {socLinks.map(({ alt, href, img }) => (
            <SocLink alt={alt} href={href} img={img} key={alt} />
          ))}
        </div>
      </div>
    </nav>
  );
};
