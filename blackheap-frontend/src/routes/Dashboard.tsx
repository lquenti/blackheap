import {Link} from 'react-router-dom'

const Dashboard = () => {
  return (<div style={{backgroundColor: '#0f0'}}>
    Dashboard
    <p>
      <Link to="/">Back to uploader</Link>
    </p>
  </div>);
};

export default Dashboard;
