import type { Metadata } from "next";

import "app-flat/styles/globals.css";
import { ralewayFont } from "app-flat/styles/fonts/index";
import { Navbar, Header } from "widgets";

export const metadata: Metadata = {
  title: "Bebrotype",
  description: "Blindtyping trainer for real bebrochads",
  icons: "/icons/banana.svg",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className={ralewayFont.className}>
      <body>
        {children}
      </body>
    </html>
  );
}
