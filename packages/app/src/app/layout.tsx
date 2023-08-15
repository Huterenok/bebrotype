"use client";

import { Suspense } from "react";

import { Providers } from "app-flat/providers";
import { Navbar, Header } from "widgets";
import { Loader } from "shared/ui";

import "app-flat/styles/globals.css";
import "react-toastify/dist/ReactToastify.css";

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <Providers>
          <div className="layout">
            <Navbar />
            <Header />
            <Suspense fallback={<Loader />}>
              <div className="pages_wrapper">{children}</div>
            </Suspense>
          </div>
        </Providers>
      </body>
    </html>
  );
}
