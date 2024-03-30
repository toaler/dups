import React from 'react'
import styled from "styled-components"
import CommitIcon from '@mui/icons-material/Commit';

function StagingHeader({ totalBytes }) {
    const bytesReclaimed = 0.0;
    const filesDeleted = 0;
    const filesCompressed = 0;

    return <StagingHeaderContainer>
        <StagingHeaderLeft>
            <h1>Bytes in scope : {totalBytes.toLocaleString("en-US")}</h1>
            <h1>Bytes reclaimed : {bytesReclaimed.toLocaleString("en-US")}</h1>
            <h1>Files deleted : {filesDeleted.toLocaleString("en-US")}</h1>
            <h1>Bytes compressed : {filesCompressed.toLocaleString("en-US")}</h1>
        </StagingHeaderLeft>
        <StagingCommit>
            // TODO : add onclick
        </StagingCommit>


    </StagingHeaderContainer>
}

export default StagingHeader

const StagingHeaderContainer = styled.div`
    display: flex;
    font-family: San Francisco;
    font-size: 12px;
    line-height: 24px;
    font-weight: 400;
    color: #FFFFFF;
`;

const StagingHeaderLeft = styled.div`
    display: flex;
    align-items: center; // Adjust vertical alignment as needed
    justify-content: space-between; // Spread out the children to use available space
    flex-wrap: wrap; // Allow items to wrap to a new line if needed
    gap: 20px; // Creates space between items
`;


const StagingCommit = styled(CommitIcon)`
    display: flex;
    font-size: 4rem !important; // Adjust the size as needed
    margin-left: auto;
    margin-right: 30px;
    color: inherit; // Or specify a color
`;