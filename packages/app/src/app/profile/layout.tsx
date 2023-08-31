"use client";

import { useAuth } from "features/Auth";

import styles from "app-flat/styles/page-styles/Profile.module.scss";

export default function ProfileLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const _ = useAuth();

  return <section className={styles.layout}>{children}</section>;
}
