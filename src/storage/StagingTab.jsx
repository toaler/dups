import DeleteIcon from '@mui/icons-material/Delete';
import StagingHeader from "./StagingHeader.jsx";
import React from "react"; // Assuming you use MUI IconButton for better click handling

function StagingTab({ actions, setActions }) {
    // Function to handle deletion of an item from the actions array
    const handleDelete = (indexToDelete) => {
        setActions(currentActions => currentActions.filter((_, index) => index !== indexToDelete));
    };

    const totalBytes = actions.reduce((acc, action) => acc + action.bytes, 0);

    return (
        <div>
            <StagingHeader totalBytes={totalBytes}></StagingHeader>
            <table>
                <thead>
                <tr>
                    <th style={{textAlign: "center"}}>Remove</th>
                    <th style={{textAlign: "left"}}>Action</th>
                    <th style={{textAlign: "left"}}>Resource</th>
                    <th style={{textAlign: "right"}}>Bytes</th>
                </tr>
                </thead>
                <tbody>
                {actions.map((actionObj, index) => (
                    <tr key={index}>
                        <td>
                            <DeleteIcon style={{padding: 0, textAlign: "center"}} onClick={() => handleDelete(index)} />
                        </td>
                        <td>{actionObj.action}</td>
                        <td>{actionObj.path}</td>
                        <td style={{textAlign: "right"}}>{actionObj.bytes.toLocaleString("en-US")}</td>
                    </tr>
                ))}
                </tbody>
            </table>
        </div>
    );
}

export default StagingTab;