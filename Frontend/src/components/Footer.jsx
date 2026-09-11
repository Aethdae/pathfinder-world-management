import React from "react";
import { footerClasses } from "../../helpers/htmlClasses";

export default function Footer() {
  return (
    <footer className={footerClasses.join(" ")}>
      Copyright Tyler Long &copy;{new Date().getFullYear()}
    </footer>
  );
}
