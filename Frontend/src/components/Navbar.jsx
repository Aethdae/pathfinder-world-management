import { Link } from "react-router";
import {
  mainNavButtonClasses,
  transitionClasses,
} from "../helpers/htmlClasses";

export default function Navbar() {
  return (
    <nav>
      <div className="flex gap-2 justify-center py-3">
        <Link
          className={
            mainNavButtonClasses.join(" ") + " " + transitionClasses.join(" ")
          }
          to={"/"}
        >
          Home
        </Link>
      </div>
    </nav>
  );
}
