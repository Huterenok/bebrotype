import { Navbar, Header } from "widgets";

import styles from "app-flat/styles/page-styles/Typing.module.css";

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className={styles.layout_wrapper}>
      <Navbar />
      <Header />
      <div className={styles.pages_wrapper}>{children}</div>
    </div>
  );
}
