import {Link} from 'react-router-dom';

const FileUploader = () => {
  return (
    <div style={{backgroundColor: "#f00"}}>
      File uploader
      <p>
        <Link to="/dashboard">go to dashboard</Link>
      </p>
    </div>
  );
}


export default FileUploader;
